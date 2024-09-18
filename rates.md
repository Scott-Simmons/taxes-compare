The start of thinking about ER adjustments. Lots of interesting ideas but for V2 will keep it simple.

Suppose base currency selected to be "NZD"

Suppose the country currencies are "USD" and "EUR"

curl  https://open.er-api.com/v6/latest/NZD | jq '.rates | {"USD","EUR"}'
