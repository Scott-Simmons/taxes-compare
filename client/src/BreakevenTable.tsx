import React, { useState, useRef, useEffect } from 'react';
import './BreakevenTable.css';
import { BackEndResponse } from './types';

interface BreakevenProps {
  data: BackEndResponse;
  currency: string | null;
}

const BreakevenData: React.FC<BreakevenProps> = ({ data, currency }) => {
  const [isFolded, setIsFolded] = useState(true);
  const tableRef = useRef<HTMLTableElement>(null);

  const toggleFold = () => {
    setIsFolded(!isFolded);
  };

  useEffect(() => {
    if (!isFolded && tableRef.current) {
      tableRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  }, [isFolded]);

  if (!data.country_comb_data) {
    return null
  }

  const tableRows = Object.entries(data.country_comb_data).map(
    ([country_comb, breakevenData]) => {
      const incomes: number[] = breakevenData.breakeven_incomes;
      const rates: number[] = breakevenData.breakeven_effective_tax_rates;
      const amounts: number[] = breakevenData.breakeven_tax_amounts;
      return incomes.map((income, index) => (
        <tr key={`${country_comb}-${index}`}>
          <td>{country_comb.split('-')[0]}</td>
          <td>{country_comb.split('-')[1]}</td>
          <td>{income.toFixed(0)}</td>
          <td>{(100.0*rates[index]).toFixed(1)}</td>
          <td>{amounts[index].toFixed(0)}</td>
        </tr>
      ));
    });

  const isEmptyArray = (arr: Array<Array<any>>): arr is any[][] => {
    return (
      (Array.isArray(arr) && arr.length > 0 && arr.every((innerArray) => Array.isArray(innerArray) && innerArray.length === 0))
    );
  };

  return (
    <div>
      <button onClick={toggleFold}>
        {isFolded ? `Show breakeven incomes for country combinations (${currency || "Local Currency"})` : `Hide breakeven points for country combinations ${currency}`}
      </button>

      {!isFolded && (
        <div className="breakeven-container">
          <table ref={tableRef} className="breakeven-table">
            <thead>
              <tr>
                <th colSpan={5}>Breakeven Points:</th>
              </tr>
              <tr>
                <th>Country 1</th>
                <th>Country 2</th>
                <th>Income where taxes are equal between countries ({currency})</th>
                <th>Taxation %</th>
                <th>Taxation Amount ({currency})</th>
              </tr>
            </thead>
            <tbody>
              {isEmptyArray(tableRows) || (Object.keys(data.country_comb_data).length === 0) ? (
                <tr>
                  <td colSpan={5}>No breakeven points found for countries</td>
                </tr>
              ) : (
                tableRows
              )}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default BreakevenData;
