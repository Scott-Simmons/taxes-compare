# Bugs Found in taxes_compare (Server)

## Fixed

### 1. `null_to_infinity` Deserializer Did Not Convert Null to Infinity
**File:** `src/core/points/marginal_rate_knot.rs`

Deserializing `Option<serde_json::Value>` caused JSON `null` to become `None` instead of `Some(f32::INFINITY)`, because serde's `Option` deserialization treats null as `None`. Fixed by deserializing into `serde_json::Value` directly.

**Status:** Fixed

---

### 2. `eprint!` Used Instead of `log::error!`
**File:** `src/controller/handle_request.rs`

Error handler used `eprint!` instead of `log::error!`, bypassing the log framework.

**Status:** Fixed

---

### 3. Server Panic on Exchange Rate API Failure
**File:** `src/controller/taxes_config.rs`, `src/exchange_rates.rs`

`.unwrap()` on the exchange rate fetch and JSON parse would panic and crash the server if the external API was unreachable or returned unexpected data. Replaced with proper error propagation.

**Status:** Fixed

---

## Previously Reported - False Positives (Removed)

- **"Missing First Knot Point"** - The production `taxes.json` format (first bracket at income_limit > 0) is correct for the algorithm. The algorithm uses implicit `(b_0=0, r_0=0)` as the base. Adding a knot at `income_limit=0` would double-count the first bracket's contribution.

- **"Improper Response Construction"** - `HttpResponse` implements `Responder` in actix-web. Returning `res` directly is valid.

- **"Potential Memory Leak in Parallel Processing"** - Rayon's parallel iterators handle memory correctly. No leak.
