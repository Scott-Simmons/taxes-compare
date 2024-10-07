import React, {useState, useRef, useEffect} from 'react';
import './IncomeTable.css'; 
import { BackEndResponse } from './types';

interface IncomeDataProps {
  data: BackEndResponse;
  income: number | null;
  currency: string | null;
}

const IncomeData: React.FC<IncomeDataProps> = ({ data, income, currency }) => {
    

    const tableRef = useRef<HTMLTableElement>(null);

    const [isFolded, setIsFolded] = useState(true);

    const toggleFold = () => {
        setIsFolded(!isFolded);
    };


    useEffect(() => {
        if (!isFolded && tableRef.current) {
          tableRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }
      }, [isFolded]);

    if (!income) {
     return null;
    }

  const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));

  // Null handling needed to keep TS happy even though null income implies null tax amounts.
  dataArray.sort((a,b) => (a.taxInfo.specific_tax_amount || 0) - (b.taxInfo.specific_tax_amount || 0));

  const rankedDataArray = dataArray.map((entry, index) => ({ ...entry, rank: index + 1 }));

  const tableRows = rankedDataArray.map((entry) => (
    <tr key={entry.country}>
      <td>{entry.country}</td>
      <td>{entry.rank}</td>
      {/*Never will be null if income is null, but need these checks to keep TS happy*/}
      <td>{((entry.taxInfo.specific_tax_rate || 0.0) * 100.0).toFixed(2)}</td>
      <td>{entry.taxInfo.specific_tax_amount?.toFixed(0)}</td>
      <td>{(income - (entry.taxInfo.specific_tax_amount || 0.0)).toFixed(0)}</td>
    </tr>
  ));

  return (
    <div>
         <button onClick={toggleFold}>
        {isFolded ? `Show taxation by country when income is ${income}` : `Hide taxation by country when income is ${income}`}
      </button>

    {!isFolded && (

    <div className="income-data-container">
      <table ref={tableRef} className="income-data-table">
        <thead>
        <tr>
          <th colSpan={5}>{`Taxation when income is ${income} (${currency || "Local Currency"})`}:</th>
        </tr>
        <tr>
          <th>Country</th>
          <th>Rank</th>
          <th>Taxation %</th>
          <th>Taxation Amount ({currency || "Local Currency"})</th>
          <th>{`Amount (from ${income}) left over`}</th>
        </tr>
        </thead>
        <tbody>{tableRows}</tbody>
      </table>
    </div>
  )}
  </div>
  )
};

export default IncomeData;
