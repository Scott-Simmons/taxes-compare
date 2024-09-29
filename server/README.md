### Backend:

See [methodology](/server/methodology.md) for more detail.

#### Examples

Example payload to send to backend
```json
{
  "countries": ["New Zealand", "Australia"],
  "income": 50000.0,
  "max_income": 200000.0,
  "show_break_even": true,
  "normalizing_currency": "NZD"
}
```

Example request:
```bash
curl -X POST http://127.0.0.1:3000/process \
     -H "Content-Type: application/json" \
     -d '{"countries":["New Zealand","Australia"],"income":50000.0,"max_income":200000.0,"show_break_even":true,"normalizing_currency":"NZD"}'
```


Example response:

```json
{
    "country_specific_data": {
        "Australia": {
            "Incomes": [
                ...
            ],
            "tax_amounts": [
                ...
            ],
            "effective_tax_rates": [
                ...
            ],
            "specific_tax_amount": 0,
            "specific_tax_rate": 0
        },
        "New Zealand": {
            "Incomes": [
                ...
            ],
            "tax_amounts": [
                ...
            ],
            "effective_tax_rates": [
                ...
            ],
            "specific_tax_amount": 0,
            "specific_tax_rate": 0
        },
    },
    "country_comb_data": {
        "New Zealand-Australia": {
            "breakeven_incomes": [
                ...
            ],
            "breakeven_tax_amounts": [
                ...
            ],
            "breakeven_effective_tax_rates": [
                ...
            ]
        }
    }
}

```


