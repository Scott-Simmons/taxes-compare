# Compare effective tax rates between countries

The easiest way to understand personal income tax is see the whole curve. Average tax rates obscure important information. Similarly, marginal tax rates tables can also be misleading.

This application gives the full picture on tax, all derived from the marginal rates of several countries.

Details on the solution [here](/server/methodology.md).

### Features

1. Visualisation of the tax curves of one or more countries (in terms of effective rates, and amount owed).

2. Conveniently lookup and view the marginal tax rate schedules of each country.

3. Analyse the tax amounts and effective rate at a specific income level.

4. Adjust the analysis to use a common currency, for true apples to apples comparisons.

5. Analyse breakeven points - i.e. the points at which the tax rates between two countries are equivalent.

### App Components

- [Frontend](/client/)
- [Backend](/server/)
- [Infrastructure](/deploy/)

Note: data for each countries tax schedules are stored [in the backend as a config file](https://github.com/Scott-Simmons/taxes-compare/blob/93868822405f519328868a45673f23643f3fb76b/server/assets/taxes.json). This data will eventually go stale if this repository is not maintained.

Contributions for updating tax schedules, adding new countries, and/or adding new currencies are welcome. 

