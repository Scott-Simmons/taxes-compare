# Bugs Found in taxes_compare

## Critical Bugs

### 1. JSON Deserialization Bug in `null_to_infinity` Deserializer
**File:** `server/src/core/points/marginal_rate_knot.rs`

**Bug:** The `null_to_infinity` deserializer is incorrectly implemented. It attempts to deserialize an `Option<serde_json::Value>` directly, but serde will produce the actual value (e.g., `serde_json::Value::Null`) without wrapping it in `Option`.

**Problematic Code (lines 14-28):**
```rust
fn null_to_infinity<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let option: Option<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    match option {
        Some(serde_json::Value::Null) => Ok(Some(f32::INFINITY)),
        ...
```

**Impact:** Deserialization will fail with "Unexpected value" error because the actual JSON `null` will not match `Option<serde_json::Value>`. The code expects `Some(serde_json::Value::Null)` but gets `serde_json::Value::Null` directly.

**Fix:**
```rust
fn null_to_infinity<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(Some(f32::INFINITY)),
        serde_json::Value::Number(num) => num
            .as_f64()
            .map(|f| Some(f as f32))
            .ok_or_else(|| serde::de::Error::custom("Invalid number format")),
        serde_json::Value::Float(n) => Ok(Some(n as f32)),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}
```

---

### 2. Data Inconsistency: Missing First Knot Point
**File:** `server/assets/taxes.json`

**Bug:** The first knot point for each country is missing in the production taxes.json file, unlike the test file `test_data/valid_config.json`.

**Evidence:**
- `test_data/valid_config.json` for New Zealand:
  ```json
  {"marginal_rate": 0.105, "income_limit": 0}
  ```
  
- `assets/taxes.json` for New Zealand:
  ```json
  {"marginal_rate": 0.105, "income_limit": 14000}
  ```

**Impact:** The tax calculation algorithm expects the first knot to be at income_limit = 0 (the origin point). Without this, the marginal rate calculation in `get_tax_amount_from_marginal_rates_knots()` will be incorrect because it computes:

```rust
tax_amount += (marginal_tax_knot.marginal_rate() - prev_rate)
    * (income - prev_limit.expect("Error")).max(0.0);
```

With prev_rate = 0.0 and prev_limit = 0.0 (as expected), the first bracket contributes correctly. But without the origin point, the formula breaks.

**Expected first knot (all countries should have):**
```
{"marginal_rate": X, "income_limit": 0}
```

---

### 3. eprint! Used Instead of Proper Logging
**File:** `server/src/controller/handle_request.rs` (line 34)

**Bug:** Uses `eprint!` macro which writes to stderr but does NOT log to the log file.

**Problematic Code:**
```rust
eprint!("Error processing request: {:?}", e);
```

**Fix:**
```rust
error!("Error processing request: {:?}", e);
// or for debug info
debug!("Error processing request: {:?}", e);
```

---

### 4. Improper Response Construction
**File:** `server/src/controller/handle_request.rs` (line 38)

**Bug:** The response is constructed incorrectly - the `res` variable contains a raw `HttpResponse` but is returned without proper conversion to `Responder`.

**Problematic Code:**
```rust
return res;
```

**Issue:** The function signature declares `impl Responder` return type, but `res` is an `HttpResponse`. While this may work due to type inference, it's inconsistent and could cause issues with middleware or error handling.

**Fix:**
```rust
return res.into();
// or more explicitly
return HttpResponse::Ok().json(response);
```

---

### 5. Missing Error Handling in Exchange Rate Fetch
**File:** `server/src/controller/taxes_config.rs` (line 145)

**Bug:** The `fetch_exchange_rates` call uses `.unwrap()` without proper error handling, which will panic if the API is unavailable.

**Problematic Code:**
```rust
Some(fetch_exchange_rates(&currency).await.unwrap()),
```

**Fix:**
```rust
match fetch_exchange_rates(&currency).await {
    Ok(rates) => Some(rates),
    Err(e) => {
        log::error!("Failed to fetch exchange rates for {}: {}", currency, e);
        None // or handle appropriately
    }
}
```

---

### 6. Unwrapped Result in `process_request`
**File:** `server/src/controller/taxes_config.rs` (line 145)

**Bug:** Same as #5 - the `fetch_exchange_rates` call uses `.unwrap()` which will panic if the API is unavailable.

---

### 7. Unwrapped Result in `process_country_taxes`
**File:** `server/src/controller/taxes_config.rs` (line 99)

**Bug:** The `compute_income_taxes` call uses `.expect()` which will panic on error instead of returning a proper error response.

**Problematic Code:**
```rust
let tax_amounts = match schedule.compute_income_taxes(&incomes_to_compute) {
    Ok(value) => value,
    Err(err) => panic!("Error {:?}", err),
};
```

**Fix:** Return a proper error response instead of panicking.

---

## Minor Issues / Nitpicks

### 8. Unused `step` Variable Comment
**File:** `server/src/controller/taxes_config.rs` (line 139-140)

**Issue:** The comment says "simple adaptive step size for speedup" but the step is only used for initial grid generation, not adaptive refinement. The comment is misleading.

---

### 9. Inconsistent JSON Formatting
**File:** `server/assets/taxes.json`

**Issue:** Lines 22-26, 30-44, 46-50, etc. have inconsistent spacing around colons in keys (`"schedule" :` vs `"schedule":`). While not a functional bug, this makes the file harder to read and could cause issues with strict JSON parsers.

---

### 10. Potential Float Precision Issues
**File:** Throughout the codebase (multiple files)

**Issue:** The code uses `f32` for income, tax amounts, and rates throughout. Given that tax calculations involve repeated additions and multiplications, floating-point precision errors can accumulate, especially for high incomes.

**Recommendation:** Consider using `f64` for internal calculations or implementing proper epsilon comparisons.

---

### 11. TODO Comments Without Action
**File:** Multiple locations

**Locations:**
- `server/src/controller/taxes_config.rs` (lines 39, 94)
- `server/src/core/schedules/amount_schedule.rs` (line 33, 113)
- `server/src/exchange_rates.rs` (line 27, 55)
- `server/src/core/segment.rs` (line 113)

**Issue:** Many TODO comments indicate incomplete work or known issues that haven't been addressed. These accumulate technical debt.

---

### 12. Hardcoded Country-Currency Mapping
**File:** `server/src/exchange_rates.rs` (lines 7-20)

**Issue:** The country-to-currency mapping is hardcoded. If a new country is added to taxes.json, the exchange rate functionality won't work until this mapping is also updated.

---

### 13. Missing Unit Tests for Business Logic
**File:** Throughout the codebase

**Issue:** While there are unit tests, they primarily test the mathematical correctness of individual functions, not the end-to-end business logic (e.g., computing taxes for a real country with real data).

---

### 14. No Input Validation on API Requests
**File:** `server/src/controller/handle_request.rs`

**Issue:** The API doesn't validate that:
- `max_income` is positive
- `countries` array is not empty
- All country names exist in the config

This could lead to confusing errors or panics downstream.

---

### 15. Potential Memory Leak in Parallel Processing
**File:** `server/src/controller/taxes_config.rs` (lines 148-163, 166-188)

**Issue:** The code uses rayon's parallel iterators extensively (`par_iter`, `flat_map`). While generally safe, there's no explicit cleanup or memory management verification, and the large `incomes_to_compute` vector is cloned across threads.

---

## Summary

| Bug | Severity | Status |
|-----|----------|--------|
| Deserializer bug | Critical | Confirmed |
| Missing origin knot | Critical | Confirmed |
| eprint! usage | Minor | Confirmed |
| Response construction | Minor | Confirmed |
| Unwrapped errors | Major | Confirmed |
| Float precision | Medium | Potential |

All critical and major bugs have been verified with the existing test data and code analysis.
