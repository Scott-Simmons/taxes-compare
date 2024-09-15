# Compare effective tax rates between countries


High level thinking rn:


Frontend Needs: 

Will be implemented in React + Typescript + Plotly because V1 did a lot of this work already.

1) A "Select countries" dropdown
2) A "Income" input box
3) A "Maximum income to consider" input box
4) A "Show breakeven points" input box
5) Function to render plot. Reduce payload size over the wire.
6) A normalising currency. Default is the locale of the client.
7) Needs to look good enough.

Backend Needs:

Will be implemented in rust for learning purposes.


```bash
curl -X POST http://127.0.0.1:8080/process \
     -H "Content-Type: application/json" \
     -d '{"countries":["New Zealand","Australia"],"income":50000.0,"max_income":200000.0,"show_break_even":true,"normalizing_currency":"NZD"}'
```

Example request
```json
{
  "countries": ["New Zealand", "Australia"],
  "income": 50000.0,
  "max_income": 200000.0,
  "show_break_even": true,
  "normalizing_currency": "NZD"
}
```

Example response

```json
{
    "country_specific_data": {
        "Australia": {
            "Incomes": [
                1,
                2,
                ...
            ],
            "tax_amounts": [
                0,
                0,
                ...
            ],
            "effective_tax_rates": [
                0,
                0,
                ...
            ],
            "specific_tax_amount": 0,
            "specific_tax_rate": 0
        },
        "New Zealand": {
            "Incomes": [
                1,
                2,
                ...
            ],
            "tax_amounts": [
                0,
                0,
                ...
            ],
            "effective_tax_rates": [
                0,
                0,
                ...
            ],
            "specific_tax_amount": 0,
            "specific_tax_rate": 0
        },
    },
    "country_comb_data": {
        "New Zealand-Australia": {
            "breakeven_incomes": [
                0,
                ...
            ],
            "breakeven_tax_amounts": [
                0,
                0
            ],
            "breakeven_effective_tax_rates": [
                0,
                ...
            ]
        }
    }
}

```


1) A list of countries (do "x" for all countries)
2) A specific income value (compute_income_tax for a specific income, doing for all countries)
3) A max income to consider (upper bound for the efficient interpolation, doing for all countries)
4) A bool saying to show breakeven points or not (if true then take all pairwise combs of countries)
5) The normalising currency e.g. NZD, AUD, or "local" where local means no normalisation, adjustment for all countries.

The response will need to have:

1) Breakeven point mapping str --> list[point] {country_comb: [breakeven_1, breakeven_2, ...]}
2) Specific Income mapping str --> point {country: point} (need to be separate for precision guarentees not relying on interpolation).
3) Marginal Rates curve str --> (list[x], list[y]) (adjusted by exchange rates)
4) Tax amounts curve str --> (list[x], list[y]) (adjusted by exchange rates)


TODO: A cool idea is to treat exchange rate as an r.v. Exchange rate is a linear transform, so can efficiently sample to get curves with upper and lower confidence bands. This would become more complex (lookback windows, persistence etc) and therefore can be part of V3.

We start with data that characterises the marginal tax rates as a function of income because this is easy to maintain.

```python
import matplotlib.pyplot as plt
import numpy as np
def marginal_tax_rate(income):
    if income <= 18200:
        return 0
    elif income <= 45000:
        return 0.16
    elif income <= 135000:
        return 0.30
    elif income <= 190000:
        return 0.37
    else:
        return 0.45
income_values = np.linspace(0, 200000, 500)
marginal_rates = [marginal_tax_rate(income) for income in income_values]
plt.figure(figsize=(10, 6))
plt.step(income_values, marginal_rates, where='post', label='Marginal Tax Rate')
plt.xlabel('Income')
plt.ylabel('Marginal Tax Rate')
plt.title('Marginal Tax Rate vs Income')
plt.legend()
plt.grid(True)
plt.ylim(0, 0.5)
plt.show()
```

We want to efficiently move to a peicewise linear characterisation of the income tax as a function of income, given our marginal rates $r_i$ and our max brackets $b_i$ for $n$ brackets.

$$x \in R, \mu \in \{(r_i, b_i), ...\}, r_0 = 0, b_0 = 0$$

$$ f(x ; \mu ) = \sum_{i=1}^{n}(r_i - r_{i - 1})max(0, x - b_{i - 1}$$

Which can be used to efficiently computed $n$ new knot points $(x_i, y_i) = (x_i, f(b_i; \mu)$ that represent the knot points of the peicewise linear funcion that defines the tax amount.

But note that the last one will be an unbounded point that needs special handling. A max income to consider must be provided.

For the set of knots, this will be sorted. It parameterises the function given we know it is linear.

To get an individual value, need to find the segment that it lies in, and then perform linear interpolation between it.

For efficient traversal for a large number of income values, function shoud do a linear scan of each of the discretised points. So that is doesn't have to search for which segment to interpolate each time.

The step size will be a big factor. It could be an adaptive step size. But this can be part of V3.

Applying the transform from tax paid to effective tax rate will also be efficient: (x,y)-->(x, y/x).

The normalisation in my view should only apply to the y axis. Tax amounts should be on same scale, but x axis should remain on its own scale.

Good references: 
1. https://math.stackexchange.com/questions/3488993/intersection-of-2-piecewise-linear-curves
2. https://publicapis.dev/category/currency-exchange

