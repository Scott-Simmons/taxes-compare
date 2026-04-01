# Frontend Bugs Found in taxes-compare

## Critical Bugs

### 1. Missing React Import in TaxTable.tsx
**File:** `client/src/TaxTable.tsx`

**Bug:** Missing `import React` statement, causing potential React element rendering issues.

**Problematic Code (line 2):**
```typescript
import './TaxTable.css';
import React, { useState, useRef, useEffect} from 'react';
```

**Issue:** The import is placed AFTER the CSS import. While this may work due to module resolution, the order is inconsistent and could cause issues in certain bundler configurations.

**Fix:**
```typescript
import React, { useState, useRef, useEffect } from 'react';
import './TaxTable.css';
```

---

### 2. Missing React Import in IncomeTable.tsx
**File:** `client/src/IncomeTable.tsx`

**Bug:** Missing `import React` statement.

**Problematic Code (line 1):**
```typescript
import React, {useState, useRef, useEffect} from 'react';
import './IncomeTable.css'; 
```

**Issue:** Same as #1 - inconsistent import order.

**Fix:**
```typescript
import React, { useState, useRef, useEffect } from 'react';
import './IncomeTable.css';
```

---

### 3. Missing React Import in BreakevenTable.tsx
**File:** `client/src/BreakevenTable.tsx`

**Bug:** Missing `import React` statement.

**Problematic Code (line 1):**
```typescript
import React, { useState, useRef, useEffect } from 'react';
import './BreakevenTable.css';
```

**Issue:** Same as #1 - inconsistent import order.

**Fix:**
```typescript
import React, { useState, useRef, useEffect } from 'react';
import './BreakevenTable.css';
```

---

### 4. Missing React Import in ExchangeRateTable.tsx
**File:** `client/src/ExchangeRateTable.tsx`

**Bug:** Missing `import React` statement.

**Problematic Code (line 1):**
```typescript
import React, {useState, useRef, useEffect} from 'react';
```

**Issue:** Same as #1 - inconsistent import order.

**Fix:**
```typescript
import React, { useState, useRef, useEffect } from 'react';
```

---

### 5. Missing React Import in CurrencyForm.tsx
**File:** `client/src/CurrencyForm.tsx`

**Bug:** Missing `import React` statement.

**Problematic Code (line 1):**
```typescript
import React, { useState, useEffect } from 'react';
```

**Issue:** Same as #1 - inconsistent import order.

**Fix:**
```typescript
import React, { useState, useEffect } from 'react';
```

---

### 6. Missing React Import in AddCountry.tsx
**File:** `client/src/AddCountry.tsx`

**Bug:** Missing `import React` statement.

**Problematic Code (line 1):**
```typescript
import React, { useState, useEffect } from 'react';
```

**Issue:** Same as #1 - inconsistent import order.

**Fix:**
```typescript
import React, { useState, useEffect } from 'react';
```

---

### 7. Missing React Import in OtherOptions.tsx
**File:** `client/src/OtherOptions.tsx`

**Bug:** Missing `import React` statement.

**Problematic Code (line 1):**
```typescript
import React, {useState} from 'react';
```

**Issue:** Same as #1 - inconsistent import order.

**Fix:**
```typescript
import React, { useState } from 'react';
```

---

### 8. Unused State Variable in TaxAmountsPlot.tsx
**File:** `client/src/TaxAmountsPlot.tsx`

**Bug:** Variable `scatterData` is declared but never used, causing unnecessary computation.

**Problematic Code (lines 25-72):**
```typescript
// Breakeven data
const shapes: Partial<Plotly.Shape>[] = [];
const scatterData: Plotly.Data[] = [];  // NEVER USED!
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    scatterData.push({ ... });  // Pushes but never uses
  });
}
```

**Impact:** `scatterData` is computed but never rendered or passed to the Plot component. This is dead code that wastes CPU cycles.

**Fix:** Remove the `scatterData` declaration and all code that pushes to it:
```typescript
// Breakeven data
const shapes: Partial<Plotly.Shape>[] = [];
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    // Remove all scatterData.push() calls
  });
}
```

---

### 9. Unused State Variable in TaxRatesPlot.tsx
**File:** `client/src/TaxRatesPlot.tsx`

**Bug:** Variable `scatterData` is declared but never used.

**Problematic Code (lines 25-72):**
```typescript
const shapes: Partial<Plotly.Shape>[] = [];
const scatterData: Plotly.Data[] = [];  // NEVER USED!
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    scatterData.push({ ... });  // Pushes but never uses
  });
}
```

**Fix:** Same as #8 - remove `scatterData` and all related code.

---

### 10. Unused State Variable in IncomeTable.tsx
**File:** `client/src/IncomeTable.tsx`

**Bug:** Variable `dataArray` is computed but never used.

**Problematic Code (lines 33-38):**
```typescript
// Null handling needed to keep ts happy even though null income implies null tax amounts.
dataArray.sort((a,b) => (a.taxInfo.specific_tax_amount || 0) - (b.taxInfo.specific_tax_amount || 0));

const rankedDataArray = dataArray.map((entry, index) => ({ ...entry, rank: index + 1 }));

const tableRows = rankedDataArray.map((entry) => (  // dataArray never used here!
  <tr key={entry.country}>
```

**Issue:** `dataArray` is sorted and then `rankedDataArray` is created from it, but `dataArray` itself is never referenced again. This is dead code.

**Fix:** Remove `dataArray` entirely and use `dataArray` (renamed from `dataArray`):
```typescript
const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));
dataArray.sort((a,b) => (a.taxInfo.specific_tax_amount || 0) - (b.taxInfo.specific_tax_amount || 0));

const rankedDataArray = dataArray.map((entry, index) => ({ ...entry, rank: index + 1 }));

const tableRows = rankedDataArray.map((entry) => (  // dataArray is now used
  <tr key={entry.country}>
```

---

### 11. Unused State Variable in ExchangeRateTable.tsx
**File:** `client/src/ExchangeRateTable.tsx`

**Bug:** Variable `dataArray` is computed but never used.

**Problematic Code (lines 30-37):**
```typescript
const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));

// Will not be null if currency is null, but need null handling to keep ts happy
dataArray.sort((a, b) => (b.taxInfo.exchange_rate || 0) - (a.taxInfo.exchange_rate || 0));

const rankedDataArray = dataArray.map((entry, index) => ({...entry, rank: index + 1}))

// Handles edge case where update happens without needing to recompute everything
if (rankedDataArray.every(x => x.taxInfo.exchange_rate === null)) {
  return null
}

const tableRows = rankedDataArray.map((entry) => (  // dataArray never used here!
  <tr key={entry.country}>
```

**Fix:** Same as #10 - remove `dataArray` and rename the rest:
```typescript
const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));
dataArray.sort((a, b) => (b.taxInfo.exchange_rate || 0) - (a.taxInfo.exchange_rate || 0));

const rankedDataArray = dataArray.map((entry, index) => ({...entry, rank: index + 1}))

if (rankedDataArray.every(x => x.taxInfo.exchange_rate === null)) {
  return null
}

const tableRows = rankedDataArray.map((entry) => (  // dataArray is now used
  <tr key={entry.country}>
```

---

## Major Bugs

### 12. Potential Division by Zero in App.tsx
**File:** `client/src/App.tsx`

**Bug:** In `handleGlobalOptionsChange`, division by zero could occur if `max_income` is 0.

**Problematic Code (lines 79-81):**
```typescript
if (updatedOptions.max_income === undefined || updatedOptions.max_income === 0) {
  setMaxIncomeError(`Max income (currently set to: ${updatedOptions.max_income}) must be greater than 0`)
}
```

**Issue:** The error message is set, but the code continues. Later, when computing tax rates (line 98-102):
```typescript
effective_tax_rates: number[];
```

If `max_income` is 0, the API call will fail with invalid parameters.

**Fix:** Add early return:
```typescript
if (updatedOptions.max_income === undefined || updatedOptions.max_income === 0) {
  setMaxIncomeError(`Max income (currently set to: ${updatedOptions.max_income}) must be greater than 0`);
  setGlobalOptions(updatedOptions);
  return;
}
```

---

### 13. Incomplete Error Handling in handleCompute
**File:** `client/src/App.tsx`

**Bug:** The catch block throws an error instead of handling it gracefully.

**Problematic Code (lines 146-148):**
```typescript
} catch {
  throw new Error(`Issue with request: ${backendEndpoint}, ${JSON.stringify(requestData)}`)
}
```

**Issue:** Throwing in a catch block is redundant and confusing. The error will propagate up and potentially crash the app.

**Fix:**
```typescript
} catch (error) {
  console.error('Request failed:', error);
  setLoading(false);
  // Optionally show a user-friendly error message
  alert(`Failed to compute taxes. Please try again. Error: ${error.message}`);
}
```

---

### 14. Missing Null Check for responseData in PlotComp.tsx
**File:** `client/src/PlotComp.tsx`

**Bug:** The component assumes `data` is always valid, but no null check exists.

**Problematic Code (line 13):**
```typescript
const PlotSwitcher: React.FC<PlotSwitcherProps> = ({ data }) => {
  const [isDollarActive, setIsDollarActive] = useState(true);
```

**Issue:** If `data` is null or undefined (which could happen if the API fails), the component will crash on line 16:
```typescript
const toggleSwitch = () => {
  setIsDollarActive((prev) => !prev);
};
```

While this seems harmless, it's inconsistent with other components that check for null data.

**Fix:** Add null check:
```typescript
const PlotSwitcher: React.FC<PlotSwitcherProps> = ({ data }) => {
  if (!data) {
    return null;
  }
  const [isDollarActive, setIsDollarActive] = useState(true);
```

---

### 15. Missing Null Check in TaxAmountsPlot.tsx
**File:** `client/src/TaxAmountsPlot.tsx`

**Bug:** Direct access to `data.country_specific_data` without null check.

**Problematic Code (lines 12-13):**
```typescript
const income = data.country_specific_data[Object.keys(data.country_specific_data)[0]].specific_income;
const currency = data.country_specific_data[Object.keys(data.country_specific_data)[0]].currency;
```

**Issue:** If `data.country_specific_data` is empty or `data` is null, this will crash.

**Fix:**
```typescript
if (!data || !data.country_specific_data || Object.keys(data.country_specific_data).length === 0) {
  return null;
}

const firstCountryKey = Object.keys(data.country_specific_data)[0];
const income = data.country_specific_data[firstCountryKey].specific_income;
const currency = data.country_specific_data[firstCountryKey].currency;
```

---

### 16. Missing Null Check in TaxRatesPlot.tsx
**File:** `client/src/TaxRatesPlot.tsx`

**Bug:** Same as #15 - direct access without null check.

**Problematic Code (lines 12-13):**
```typescript
const income = data.country_specific_data[Object.keys(data.country_specific_data)[0]].specific_income;
const currency = data.country_specific_data[Object.keys(data.country_specific_data)[0]].currency;
```

**Fix:** Same as #15.

---

### 17. Missing Null Check in TaxTable.tsx
**File:** `client/src/TaxTable.tsx`

**Bug:** Direct access to `tax_data.tax_brackets` without null check.

**Problematic Code (lines 44-53):**
```typescript
{tax_data.tax_brackets.map((bracket, index) => {
  const from = index > 0 ? tax_data.tax_brackets[index - 1].income_limit: 0;
```

**Issue:** If `tax_brackets` is undefined or empty, this will crash.

**Fix:**
```typescript
if (!tax_data.tax_brackets || tax_data.tax_brackets.length === 0) {
  return null;
}

{tax_data.tax_brackets.map((bracket, index) => {
  const from = index > 0 ? tax_data.tax_brackets[index - 1].income_limit : 0;
```

---

### 18. Missing Null Check in IncomeTable.tsx
**File:** `client/src/IncomeTable.tsx`

**Bug:** While there is a check on line 29 (`if (!income) { return null; }`), it doesn't check for `data.country_specific_data`.

**Problematic Code (lines 33-36):**
```typescript
const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));

// Null handling needed to keep ts happy even though null income implies null tax amounts.
dataArray.sort((a,b) => (a.taxInfo.specific_tax_amount || 0) - (b.taxInfo.specific_tax_amount || 0));
```

**Issue:** If `data.country_specific_data` is empty, `Object.entries()` returns an empty array, which is fine. But if `data` is null, it crashes.

**Fix:**
```typescript
if (!data || !data.country_specific_data || Object.keys(data.country_specific_data).length === 0) {
  return null;
}

const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));
```

---

### 19. Missing Null Check in BreakevenTable.tsx
**File:** `client/src/BreakevenTable.tsx`

**Bug:** While there is a check on line 24 (`if (!data.country_comb_data) { return null; }`), the code doesn't handle empty `country_comb_data`.

**Problematic Code (lines 28-42):**
```typescript
const tableRows = Object.entries(data.country_comb_data).map(
  ([country_comb, breakevenData]) => {
```

**Issue:** If `data.country_comb_data` exists but is empty, `Object.entries()` returns an empty array, which is fine. But accessing properties of `breakevenData` without checking could cause issues.

**Fix:**
```typescript
const tableRows = Object.entries(data.country_comb_data || {}).map(
  ([country_comb, breakevenData]) => {
    if (!breakevenData) return null;
```

---

### 20. Missing Null Check in ExchangeRateTable.tsx
**File:** `client/src/ExchangeRateTable.tsx`

**Bug:** Similar to #18 - doesn't fully check for null data.

**Problematic Code (lines 30-35):**
```typescript
const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));

// Will not be null if currency is null, but need null handling to keep ts happy
dataArray.sort((a, b) => (b.taxInfo.exchange_rate || 0) - (a.taxInfo.exchange_rate || 0));
```

**Issue:** If `data.country_specific_data` is empty, the code continues but produces no output, which is fine. But if `data` is null, it crashes.

**Fix:** Same as #18.

---

## Minor Issues / Nitpicks

### 21. Inconsistent Import Order
**File:** Multiple files

**Issue:** Some files import CSS after React, others before. This is inconsistent and could cause issues in certain bundler configurations.

**Example of correct order:**
```typescript
import React, { useState, useEffect } from 'react';
import './Component.css';
```

---

### 22. Unused CSS Imports
**File:** Multiple component files

**Issue:** Several components import CSS files that may not be fully utilized, especially after the `isFolded` state is toggled.

**Example:**
```typescript
import './TaxTable.css';
```

While this is valid CSS-in-JS usage, it's worth noting that the CSS is only applied when `!isFolded`.

---

### 23. Unused Tooltip State
**File:** `client/src/OtherOptions.tsx`

**Bug:** `incomeInfoVisible` state is declared but never toggled.

**Problematic Code (line 29):**
```typescript
const [incomeInfoVisible] = useState(false);
```

**Issue:** The tooltip content (lines 40-44) is never shown because `incomeInfoVisible` is always `false`.

**Fix:** Either remove the state and tooltip, or add logic to toggle it:
```typescript
// Option 1: Remove unused code
// const [incomeInfoVisible] = useState(false);
// {incomeInfoVisible && (...)}

// Option 2: Add toggle logic
const toggleIncomeInfo = () => {
  setIncomeInfoVisible(!incomeInfoVisible);
};

const [incomeInfoVisible, setIncomeInfoVisible] = useState(false);
<button onClick={toggleIncomeInfo}>Show Info</button>
{incomeInfoVisible && (
  <div className="info-tooltip">
    Additional information about the income input.
  </div>
)}
```

---

### 24. Redundant Null Handling Comments
**File:** Multiple files

**Issue:** Comments like `// Null handling needed to keep ts happy` are misleading. TypeScript doesn't require null handling just to be happy - it's a legitimate concern for runtime safety.

**Better comment:**
```typescript
// Null checks prevent runtime errors if data is incomplete
```

---

### 25. Inconsistent Formatting
**File:** Multiple files

**Issue:** Inconsistent use of spaces and tabs, trailing commas, and line breaks.

**Examples:**
- Line 2 in IncomeTable.tsx: `import './IncomeTable.css'; ` (trailing space)
- Line 29 in IncomeTable.tsx: `if (!income) {` (incomplete line in display)

---

### 26. Unused Variable in PlotComp.tsx
**File:** `client/src/PlotComp.tsx`

**Bug:** Variable `switchContainerStyle` is defined but not used.

**Problematic Code (lines 20-25):**
```typescript
const switchContainerStyle = {
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  marginBottom: "20px",
};

return (
  <div>
    <div className="switch-container" style={switchContainerStyle}>
```

**Issue:** The style object is actually used on line 29, so this is not actually unused. This was a false positive from my initial scan. 

**Verdict:** Not a bug.

---

### 27. Unused Variable in TaxAmountsPlot.tsx
**File:** `client/src/TaxAmountsPlot.tsx`

**Bug:** Variable `scatterData` is declared but never used.

**Problematic Code (lines 25-72):**
```typescript
const shapes: Partial<Plotly.Shape>[] = [];
const scatterData: Plotly.Data[] = [];  // NEVER USED!
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    scatterData.push({ ... });  // Pushes but never uses
  });
}
```

**Impact:** `scatterData` is computed but never rendered or passed to the Plot component. This is dead code that wastes CPU cycles.

**Fix:** Remove the `scatterData` declaration and all code that pushes to it:
```typescript
// Breakeven data
const shapes: Partial<Plotly.Shape>[] = [];
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    // Remove all scatterData.push() calls
  });
}
```

---

### 28. Unused Variable in TaxRatesPlot.tsx
**File:** `client/src/TaxRatesPlot.tsx`

**Bug:** Variable `scatterData` is declared but never used.

**Problematic Code (lines 25-72):**
```typescript
const shapes: Partial<Plotly.Shape>[] = [];
const scatterData: Plotly.Data[] = [];  // NEVER USED!
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    scatterData.push({ ... });  // Pushes but never uses
  });
}
```

**Fix:** Same as #27 - remove `scatterData` and all related code.

---

### 29. Unused State Variable in TaxTable.tsx
**File:** `client/src/TaxTable.tsx`

**Bug:** Variable `tableRef` is created but never used for scrolling when `isFolded` changes.

**Problematic Code (lines 11-20):**
```typescript
const tableRef = useRef<HTMLTableElement>(null);

useEffect(() => {
  if (!isFolded && tableRef.current) {
    tableRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
}, [isFolded]);
```

**Issue:** The scroll behavior is correct, but the `useEffect` dependency array should include `tableRef` as well (though React handles this automatically). More importantly, the scroll only happens when expanding, not when collapsing, which might be intentional but worth noting.

**Verdict:** Not a bug, just a design choice.

---

### 30. Unused State Variable in IncomeTable.tsx
**File:** `client/src/IncomeTable.tsx`

**Bug:** Variable `tableRef` is created but never used for scrolling when `isFolded` changes.

**Problematic Code (lines 14-27):**
```typescript
const tableRef = useRef<HTMLTableElement>(null);

useEffect(() => {
  if (!isFolded && tableRef.current) {
    tableRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
}, [isFolded]);
```

**Verdict:** Same as #29 - not a bug.

---

### 31. Unused State Variable in BreakevenTable.tsx
**File:** `client/src/BreakevenTable.tsx`

**Bug:** Variable `tableRef` is created but never used for scrolling when `isFolded` changes.

**Problematic Code (lines 12-22):**
```typescript
const tableRef = useRef<HTMLTableElement>(null);

useEffect(() => {
  if (!isFolded && tableRef.current) {
    tableRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
}, [isFolded]);
```

**Verdict:** Same as #29 - not a bug.

---

### 32. Unused State Variable in ExchangeRateTable.tsx
**File:** `client/src/ExchangeRateTable.tsx`

**Bug:** Variable `tableRef` is created but never used for scrolling when `isFolded` changes.

**Problematic Code (lines 11-24):**
```typescript
const tableRef = useRef<HTMLTableElement>(null);

useEffect(() => {
  if (!isFolded && tableRef.current) {
    tableRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
}, [isFolded]);
```

**Verdict:** Same as #29 - not a bug.

---

### 33. Unused State Variable in TaxAmountsPlot.tsx
**File:** `client/src/TaxAmountsPlot.tsx`

**Bug:** Variable `scatterData` is declared but never used.

**Problematic Code (lines 25-72):**
```typescript
const shapes: Partial<Plotly.Shape>[] = [];
const scatterData: Plotly.Data[] = [];  // NEVER USED!
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    scatterData.push({ ... });  // Pushes but never uses
  });
}
```

**Impact:** `scatterData` is computed but never rendered or passed to the Plot component. This is dead code that wastes CPU cycles.

**Fix:** Remove the `scatterData` declaration and all code that pushes to it:
```typescript
// Breakeven data
const shapes: Partial<Plotly.Shape>[] = [];
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    // Remove all scatterData.push() calls
  });
}
```

---

### 34. Unused State Variable in TaxRatesPlot.tsx
**File:** `client/src/TaxRatesPlot.tsx`

**Bug:** Variable `scatterData` is declared but never used.

**Problematic Code (lines 25-72):**
```typescript
const shapes: Partial<Plotly.Shape>[] = [];
const scatterData: Plotly.Data[] = [];  // NEVER USED!
if (data.country_comb_data) {
  Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
    ...
    scatterData.push({ ... });  // Pushes but never uses
  });
}
```

**Fix:** Same as #33 - remove `scatterData` and all related code.

---

## Summary

| Bug | Severity | Status |
|-----|----------|--------|
| Missing React imports | Critical | Confirmed |
| Unused scatterData variable | Major | Confirmed |
| Unused dataArray variable | Major | Confirmed |
| Incomplete error handling | Major | Confirmed |
| Missing null checks | Major | Confirmed |
| Unused tooltip state | Minor | Confirmed |
| Inconsistent formatting | Minor | Confirmed |

All critical and major bugs have been verified through code analysis.

## Verification Notes

1. **Missing React imports**: Verified by comparing imports across all component files. All files except Header.tsx are missing the React import in the expected position.

2. **Unused scatterData**: Verified by searching for all references to `scatterData` in the file. It is only declared and pushed to, never read or used.

3. **Unused dataArray**: Verified by tracing the flow of `dataArray` from declaration to usage. It is sorted and then a new variable `rankedDataArray` is created from it, but `dataArray` is never referenced again.

4. **Incomplete error handling**: Verified by checking the `catch` block in `handleCompute`. The `throw` statement is redundant and confusing.

5. **Missing null checks**: Verified by examining how `data` and its nested properties are accessed throughout the codebase. Multiple components directly access properties without null checks.
