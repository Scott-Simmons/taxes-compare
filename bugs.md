# Verified Bugs in taxes_compare

Each bug below has been verified against the source code or authoritative tax data.

---

## 1. Scale Factor Error in Income Table

**File:** `client/src/IncomeTable.tsx` line 45

**Bug:** The effective tax rate is displayed as a raw decimal instead of a percentage. The column header says "Taxation %" but the value is not multiplied by 100.

```tsx
// IncomeTable.tsx:45 -- WRONG: shows "0.15" under "Taxation %" header
<td>{entry.taxInfo.specific_tax_rate?.toFixed(2)}</td>
```

The BreakevenTable correctly multiplies by 100:

```tsx
// BreakevenTable.tsx:38 -- CORRECT
<td>{(100.0*rates[index]).toFixed(1)}</td>
```

**Impact:** A 15% effective tax rate displays as "0.15" instead of "15.00" in the income comparison table.

**Fix:** `<td>{(entry.taxInfo.specific_tax_rate ? (entry.taxInfo.specific_tax_rate * 100).toFixed(2) : null)}</td>`

---

## 2. "null" Displayed in Breakeven Table UI Text

**File:** `client/src/BreakevenTable.tsx` lines 53, 66, 68

**Bug:** When no normalizing currency is selected (`currency` is `null`), the string "null" is rendered literally in the UI.

```tsx
// Line 53: Hide button shows "Hide breakeven points for country combinations null"
{isFolded ? `Show breakeven incomes for country combinations (${currency || "Local Currency"})` : `Hide breakeven points for country combinations ${currency}`}

// Line 66: column header shows "...between countries (null)"
<th>Income where taxes are equal between countries ({currency})</th>

// Line 68: column header shows "Taxation Amount (null)"
<th>Taxation Amount ({currency})</th>
```

The "show" button correctly uses `${currency || "Local Currency"}` but the "hide" button and column headers don't.

---

## 3. Spain Tax Data Uses Savings Tax Instead of Income Tax

**File:** `server/assets/taxes.json` (Spain entry)

**Bug:** The tax brackets for Spain are the **savings/capital gains tax** (rentas del ahorro), not the **income tax** (IRPF).

Current data in `taxes.json`:
| Rate | Income Limit | Matches |
|------|-------------|---------|
| 19%  | 6,000       | Savings tax bracket 1 |
| 21%  | 50,000      | Savings tax bracket 2 |
| 23%  | 200,000     | Savings tax bracket 3 |
| 27%  | 300,000     | Savings tax bracket 4 |
| 28%  | unbounded   | Savings tax bracket 5 |

Spain's actual **income tax** (IRPF) brackets:
| Rate | Income Limit |
|------|-------------|
| 19%  | 12,450      |
| 24%  | 20,200      |
| 30%  | 35,200      |
| 37%  | 60,000      |
| 45%  | 300,000     |
| 47%  | unbounded   |

**Impact:** Spain shows dramatically lower tax rates than reality. At EUR 50,000 income the tool shows ~21% effective rate when the real income tax effective rate is ~27%.

---

## 4. Norway Tax Data Missing Bracket Tax (trinnskatt)

**File:** `server/assets/taxes.json` (Norway entry)

**Bug:** Norway's data only includes the 22% flat rate on ordinary income (alminnelig inntekt). The bracket tax (trinnskatt), which is a separate **income tax** component (not a social contribution), is missing entirely.

Current data: single 22% flat rate.

Missing bracket tax (trinnskatt) schedule:
| Rate  | Income Threshold (NOK) |
|-------|----------------------|
| +1.7% | 208,050             |
| +4.0% | 292,850             |
| +13.6%| 670,000             |
| +16.6%| 937,900             |
| +17.6%| 1,350,000           |

**Impact:** The combined income tax rate ranges from 22% to 39.6%, but the tool shows a flat 22% at all income levels. Norway appears to be one of the lowest-taxed countries when it's actually mid-to-high.

---

## 5. Server Panic on Malformed Exchange Rate API Response

**File:** `server/src/exchange_rates.rs` line 30

**Bug:** `unwrap()` on JSON deserialization of the external API response. If the exchange rate API returns an unexpected format, HTML error page, or rate-limited response, the server panics.

```rust
let rates: ExchangeRatesResponse = serde_json::from_str(&resp).unwrap(); // panics
```

---

## 6. Server Panic When Exchange Rate API is Unreachable

**File:** `server/src/controller/taxes_config.rs` line 145

**Bug:** `unwrap()` on the async exchange rate fetch. If the API at `open.er-api.com` is down, times out, or returns an HTTP error, the server panics.

```rust
Some(fetch_exchange_rates(&currency).await.unwrap()), // panics
```

---

## 7. Error Logging Bypasses Log Framework

**File:** `server/src/controller/handle_request.rs` line 34

**Bug:** Uses `eprint!` instead of `log::error!`. The `log` crate is already imported and used (`use log::info;` on line 3, `info!` on line 27). Errors go to stderr but not through the logging framework, so they won't appear in structured logs or be captured by log aggregation.

```rust
eprint!("Error processing request: {:?}", e);  // should be: log::error!(...)
```

---

## 8. Frontend Swallows API Errors With No User Feedback

**File:** `client/src/App.tsx` lines 146-148

**Bug:** The catch block re-throws a new error instead of showing it to the user. This results in an unhandled promise rejection. The loading spinner stops (thanks to `finally`) but the user gets no indication that the request failed.

```tsx
} catch {
    throw new Error(`Issue with request: ${backendEndpoint}, ${JSON.stringify(requestData)}`)
}
```

---

## 9. Validation in handleGlobalOptionsChange Is Ineffective

**File:** `client/src/App.tsx` lines 65-90

**Bug:** The unconditional `setGlobalOptions(updatedOptions)` on line 89 always runs, overriding the conditional validation above it. Invalid values (e.g., income > max_income) are saved to state even when error messages are displayed.

```tsx
const handleGlobalOptionsChange = (options: Partial<GlobalOptions>) => {
    const updatedOptions = { ...globalOptions, ...options };

    if (... valid ...) {
      setGlobalOptions(updatedOptions);   // line 67: conditional set
      setIncomeError(null);
    } else {
      setIncomeError("...");              // error shown...
    }

    // ... more validation ...

    setGlobalOptions(updatedOptions);     // line 89: ALWAYS runs, overrides above
};
```

The `handleCompute` function has its own validation that prevents actual computation with invalid values, so this doesn't lead to incorrect results. But the validation feedback is misleading: errors appear while the invalid values are accepted.

---

## 10. PlotSwitcher Variable Name Inverted

**File:** `client/src/PlotComp.tsx` lines 14, 40

**Bug:** `isDollarActive` is `true` by default but shows the **percentage/rates** view, not the dollar/amounts view. The name is the opposite of the behavior.

```tsx
const [isDollarActive, setIsDollarActive] = useState(true);  // default: true
// ...
{isDollarActive ? (<TaxRatesPlot data={data}/>) : (<TaxAmountsPlot data={data}/>)}
//                  ^^ shows % when "dollar active"     ^^ shows $ when "dollar inactive"
```

The button highlights `%` when `isDollarActive` is true. Functionally the toggle works for the user, but the code logic is inverted from its naming.

---

## 11. `null_to_infinity` Deserializer Does Not Convert Null to Infinity

**File:** `server/src/core/points/marginal_rate_knot.rs` lines 14-28

**Bug:** The custom serde deserializer `null_to_infinity` is intended to convert JSON `null` to `Some(f32::INFINITY)`. It doesn't. When serde deserializes `null` into `Option<serde_json::Value>`, it produces `None` (not `Some(Value::Null)`), so the `Some(serde_json::Value::Null)` match arm is unreachable dead code. JSON `null` falls through to the `None => Ok(None)` branch, resulting in `income_limit = None` instead of `Some(f32::INFINITY)`.

```rust
let option: Option<serde_json::Value> = Deserialize::deserialize(deserializer)?;
match option {
    Some(serde_json::Value::Null) => Ok(Some(f32::INFINITY)),  // UNREACHABLE for JSON null
    // ...
    None => Ok(None),  // <-- JSON null lands HERE
}
```

**No runtime impact:** The code happens to work because the last bracket (the only one with null income_limit) is always handled specially via `i == self.schedule.len() - 1` checks that prevent the `None` from being unwrapped. But the function does not do what its name says.

---

## 12. Piecewise Linear Interpolation Formula Wrong in Methodology

**File:** `server/methodology.md` lines 72-74

**Bug:** The piecewise linear interpolation formula has the variables `b` (income limit) and `t` (tax amount) swapped throughout. The written formula computes the **inverse** function (income from tax amount) instead of the intended function (tax amount from income).

Documented formula (wrong):
$$g(x) = b_{i-1} + \frac{b_{i} - b_{i-1}}{t_{i} - t_{i-1}} \cdot (x - t_{i-1})$$

Correct formula (and what the code implements):
$$g(x) = t_{i-1} + \frac{t_{i} - t_{i-1}}{b_{i} - b_{i-1}} \cdot (x - b_{i-1})$$

The code in `segment.rs:44-49` is correct; only the documentation formula is wrong.

---

## Summary

| # | Bug | Location | Severity |
|---|-----|----------|----------|
| 1 | Tax rate scale factor (decimal vs percentage) | IncomeTable.tsx:45 | High (wrong data shown) |
| 2 | "null" rendered in UI text | BreakevenTable.tsx:53,66,68 | Medium (cosmetic) |
| 3 | Spain: savings tax instead of income tax | taxes.json | High (wrong data) |
| 4 | Norway: missing bracket tax | taxes.json | High (wrong data) |
| 5 | Server panic on bad API response | exchange_rates.rs:30 | High (crashes server) |
| 6 | Server panic when API unreachable | taxes_config.rs:145 | High (crashes server) |
| 7 | eprint! bypasses log framework | handle_request.rs:34 | Low (ops issue) |
| 8 | API errors not shown to user | App.tsx:146-148 | Medium (bad UX) |
| 9 | Validation always overridden | App.tsx:67,89 | Low (compute re-validates) |
| 10 | isDollarActive naming inverted | PlotComp.tsx:14,40 | Low (code readability) |
| 11 | null_to_infinity is dead code | marginal_rate_knot.rs:14-28 | Low (no runtime impact) |
| 12 | Formula wrong in methodology | methodology.md:72-74 | Low (docs only) |
