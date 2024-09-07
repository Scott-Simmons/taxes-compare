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

1) Static data holding the tax rates.
2) Pre-computed tax values for each level of income? Maybe...
3) Exchange rates normalisation. Most recent value, mean, stdev. This should be a time series.
    Exchange rate API.
    Fetch historical...
    Persist to a DB.
    90 day lookback window.
    Need to think about the details here.
    Mabye cacheing for popular currency pairs.
4) Efficient computation of the effective tax rates. Step size etc. There might be a super fast approach.
    Essentially how to compute peicewise linear functions effectively.
    May have to think about precomputed values too.
    May have to think about vectorisation.
    Likely within a "piece" linear interpolation would be the fastest way to compute. Assuming its optimised.
    Should absolutely not be doing a "search per query" just need to do one sweep.

5) Need to think carefully about (Marginal tax rate as a function of income... peicewise linear). Effective tax amount as a function of income (peicewise linear). Effective tax rate as a fucntion of income (not peicewise linear).

I think it should plot: Marginal tax rate as a function of income, tax as a function of income, effective tax rate as a function of income.







Some thinking:

We start with data that characterises the marginal tax rates as a function of income, e.g:

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

But note that the last one will be an unbounded point that needs special handling.

For the set of knots, this can be sorted. It parameterises the function given we know it is linear.

To get an individual value, need to find the segment that it lies in, and then perform linear interpolation between it.

For efficient traversal for a large number of income values, function shoud do a linear scan of each of the discretised points. So that is doesn't have to search for which segment to interpolate each time.

Should also implement a version that does not traverse, but does a binary search on the segments to find the right segment to interpolate with.

The step size will be a big factor. Do not want it to be too big nor too small.

Applying exchange rates will be a simple linear scaling of this. But the complication is that I want a distribution of curves. For example, applying current exchange rate, 90 day average exchange rate, and lastly I want some idea of the variance in the last 90 days. That means a sample of the exchange rates. But exchange rate is a linear scaling, so it can come out the front. e.g. since its linear I can just do R_low\*f(other_vars), R_high\*f(other_vars). That is lucky otherwise if R was not linear part of f then I would have to simulate multiple realisations of f and then compute confints that way. However, because that is not the case, I can efficiently update R_low, R_high, R_mean, R_most_recent each day. Because the values change per day, I can pop older one off the queue and add the new datapoint, then easily update R.

Applying the transform from tax paid to effective tax rate will also be efficient: (x,y)-->(x, y/x).

Another problem to solve will be when solving the intersection between two peicewise linear functions. https://stackoverflow.com/questions/54750349/finding-the-intersection-of-two-piecewise-linear-function

The peicewise linear functions are both monotonically increasing. https://stackoverflow.com/questions/54750349/finding-the-intersection-of-two-piecewise-linear-function

From someone on stack exchange: "You don't need to take all pairwise combinations of segments, just the overlapping ones. Start with the first segment on each polyline, then consider the next vertex of each. Whichever vertex has the lower x value, advance that polyline to the next segment. If your polylines have k1 and k2 segments, respectivement, this is O(k1 + k2) not O(k1 * k2) as it would be to test all pairs."

https://math.stackexchange.com/questions/3488993/intersection-of-2-piecewise-linear-curves

The approach: Two pointers to only process overlapping segments. Finding intersection by setting up two systems of linear equations.

