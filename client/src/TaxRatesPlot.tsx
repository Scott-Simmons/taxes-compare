import React from "react";
import { BackEndResponse } from "./types";
import Plot from 'react-plotly.js';

interface TaxRatesPlotProps {
  data: BackEndResponse;
}

const TaxRatesPlot: React.FC<TaxRatesPlotProps> = ({ data }) => {

  // To ensure values are in sync with the compute cycle, not user input in the client...
  const income = data.country_specific_data[Object.keys(data.country_specific_data)[0]].specific_income;
  const currency = data.country_specific_data[Object.keys(data.country_specific_data)[0]].currency;

  // Main plot data
  const chartData = Object.entries(data.country_specific_data).map(([countryKey, countryData]) => ({
    x: countryData.incomes,
    y: countryData.effective_tax_rates,
    mode: 'lines',
    name: countryKey,
  }));

  // Breakeven data
  let shapes: Partial<Plotly.Shape>[] = [];
  let scatterData: any[] = [];
  if (data.country_comb_data) {
    Object.entries(data.country_comb_data).flatMap(([country_comb_key, pointsData]) => {
      const incomes = pointsData.breakeven_incomes;
      const rates = pointsData.breakeven_effective_tax_rates;
      incomes.forEach((income, index) => {
        const rate = rates[index];
        shapes.push(
          {
            type: 'line',
            x0: income,
            y0: 0,
            x1: income,
            y1: rate,
            line: {
              color: 'black',
              width: 1,
              dash: 'dot'
            },
          },
          {
            type: 'line',
            x0: 0,
            y0: rate,
            x1: income,
            y1: rate,
            line: {
              color: 'black',
              width: 1,
              dash: 'dot'
            },
          }
        );
        scatterData.push({
          type: 'scatter',
          mode: 'markers',
          x: [income],
          y: [rate],
          marker: {
            color: 'black',
            size: 4,
          },
          name: `${country_comb_key} Breakeven (#${index + 1})`,
          showlegend: false,
          });
      });
    });
  };

  // Specific income data
  if (income) {
    Object.entries(data.country_specific_data).flatMap(([country_key, incomeData]) => {
      const rate = incomeData.specific_tax_rate;
        shapes.push(
          {
            type: 'line',
            x0: income,
            y0: 0,
            x1: income,
            y1: rate,
            line: {
              color: 'purple',
              width: 1,
              dash: 'dot'
            },
          },
          {
            type: 'line',
            x0: 0,
            y0: rate,
            x1: income,
            y1: rate,
            line: {
              color: 'purple',
              width: 1,
              dash: 'dot'
            },
          }
        );
        scatterData.push({
          type: 'scatter',
          mode: 'markers',
          x: [income],
          y: [rate],
          marker: {
            color: 'purple',
            size: 4,
          },
          name: `${country_key} income @ ${income})`,
          showlegend: false,
          });
      });
  };

  const plotData = [...chartData, ...scatterData];
    return (
    <div style={{
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      height: '100vh',
      width: '100%',
    }}>
      <Plot
        data={plotData}
        layout={{
          title: {
            text: `<b>Effective Tax Rates</b> (%)<br>(in ${currency || "Local Currency"})`,
            x: 0.5,
            xanchor: 'center',
          },
          xaxis: { title: `<b>Annual Taxable Income</b> (${currency || "Local Currency"})`, gridcolor: 'rgba(255, 255, 255, 0)', range: [0, undefined] },
          yaxis: { title: '<b>Effective Tax Rate</b> (%)', tickformat: '.0%', gridcolor: 'rgba(255, 255, 255, 0)', range: [0, undefined] },
          showlegend: true,
          shapes: shapes?.flat() as Partial<Plotly.Shape>[],
          font: {
            family: 'Arial, sans-serif',
            size: 14,
            color: '#333',
          },
          width: window.innerWidth * 0.8,
          height: window.innerHeight * 0.7,
        }}
      />
    </div>
  );
};

export default TaxRatesPlot;

