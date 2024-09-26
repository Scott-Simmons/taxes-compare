
export interface TaxBracket {
  marginal_rate: number;
  income_limit: number;
};

export interface BackEndResponse {

  country_comb_data: {
    [country_comb_key: string]: {
      breakeven_incomes: number[];
      breakeven_tax_amounts: number[];
      breakeven_effective_tax_rates: number[];
    };
  } | null;

  country_specific_data: {
    [country_key: string]: {
      effective_tax_rates: number[];
      incomes: number[];
      specific_tax_amount: number | null;
      specific_tax_rate: number | null;
      tax_amounts: number[];
      tax_brackets: TaxBracket[];
      exchange_rate: number | null;
      currency: string | null;
      specific_income: number | null;
    }
  };
};
