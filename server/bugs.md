# Bugs Found in taxes_compare

All previously identified bugs have been fixed.

## Fixed

### Server

1. **`null_to_infinity` deserializer was dead code** (`src/core/points/marginal_rate_knot.rs`) - JSON `null` became `None` instead of `Some(f32::INFINITY)`. Fixed by deserializing into `Value` directly.

2. **`eprint!` instead of `log::error!`** (`src/controller/handle_request.rs`) - Errors bypassed the logging framework.

3. **Server panic on exchange rate API failure** (`src/exchange_rates.rs`, `src/controller/taxes_config.rs`) - `.unwrap()` on API fetch and JSON parse. Replaced with proper error propagation.

4. **Spain tax data was savings tax, not income tax** (`assets/taxes.json`) - Rates (19/21/23/27/28%) were rentas del ahorro. Replaced with IRPF brackets (19/24/30/37/45/47%).

5. **Norway missing bracket tax** (`assets/taxes.json`) - Only had 22% flat rate, missing trinnskatt (1.7%-17.6% at higher incomes).

6. **Methodology formula had b/t swapped** (`methodology.md`) - Piecewise linear interpolation formula computed the inverse function. Code was correct.

### Client

7. **IncomeTable scale factor error** (`src/IncomeTable.tsx`) - Effective tax rate displayed as decimal (0.15) instead of percentage (15.00) under "Taxation %" column.

8. **BreakevenTable null currency** (`src/BreakevenTable.tsx`) - Hide button and column headers rendered "null" when no currency was selected.

9. **Validation always overridden** (`src/App.tsx`) - `setGlobalOptions()` ran unconditionally after validation, saving invalid values to state.

10. **Error swallowed in handleCompute** (`src/App.tsx`) - Catch block re-threw error with no user feedback. Replaced with alert.

11. **`isDollarActive` naming inverted** (`src/PlotComp.tsx`) - `true` showed percentage view, not dollar view. Renamed to `isRatesView`.

## Previously Reported - False Positives (Removed)

- **"Missing First Knot Point"** - The production `taxes.json` format is correct. The algorithm uses implicit `(b_0=0, r_0=0)` as the base.
- **"Improper Response Construction"** - `HttpResponse` implements `Responder` in actix-web.
- **"Potential Memory Leak in Parallel Processing"** - Rayon handles memory correctly.
