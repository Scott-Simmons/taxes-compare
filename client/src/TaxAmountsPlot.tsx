import React from "react";
import { BackEndResponse } from "./types";
import Plot from 'react-plotly.js';

interface TaxAmountsPlotProps {
  data: BackEndResponse;
}

const TaxAmountsPlot: React.FC<TaxAmountsPlotProps> = ({ data }) => {

  // TODO: Income lines, Breakeven Points.
  // TODO: Formatting, centering.

  const chartData = Object.entries(data.country_specific_data).map(([countryKey, countryData]) => ({
    x: countryData.incomes,
    y: countryData.tax_amounts,
    mode: 'lines',
    name: countryKey,
  }));

  return (
      <Plot
        data={chartData}
        layout={{
          title: 'Income vs Tax Amounts',
          xaxis: { title: 'Income' },
          yaxis: { title: 'Tax Amount' },
          showlegend: true,
        }}
      />
    );
    
};

export default TaxAmountsPlot;
