import React, { useState, useRef, useEffect } from 'react';
import './BreakevenTable.css';

interface BreakevenProps {
  rawData: {
    [country_combination_key: string]: {
      breakeven_income: number;
      breakeven_rate: number;
      breakeven_amount: number;
    }[];
  } | null | undefined;
  countries: string[] | null | undefined;
}

const BreakevenData: React.FC<BreakevenProps> = ({ rawData, countries}) => {
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

  if (!rawData) {
    return null;
  }

  if (rawData === undefined) {
    return null;
  }

  const tableRows = Object.entries(rawData).map(([country_comb, breakevenInfoList]) =>
    breakevenInfoList.map((breakevenInfo, index) => (
      <tr key={`${country_comb}-${index}`}>
        <td>{country_comb.split('||')[0]}</td>
        <td>{country_comb.split('||')[1]}</td>
        <td>{breakevenInfo.breakeven_income.toFixed(0)}</td>
        <td>{breakevenInfo.breakeven_rate.toFixed(1)}</td>
        <td>{breakevenInfo.breakeven_amount.toFixed(0)}</td>
      </tr>
    ))
  );

  const isEmptyArray = (arr: Array<Array<any>>): arr is any[][] => {
    return (
      (Array.isArray(arr) && arr.length > 0 && arr.every((innerArray) => Array.isArray(innerArray) && innerArray.length === 0))
    );
    
  };

  console.log(tableRows);

  return (
    <div>
      <button onClick={toggleFold}>
        {isFolded ? `Show breakeven incomes for country combinations` : `Hide breakeven points for country combinations`}
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
                <th>Income where taxes are equal between countries %</th>
                <th>Taxation %</th>
                <th>Taxation Amount (local currency)</th>
              </tr>
            </thead>
            <tbody>
              {isEmptyArray(tableRows) || (countries && countries.length === 1) ? (
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
