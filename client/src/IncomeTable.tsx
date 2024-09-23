import React, {useState, useRef, useEffect} from 'react';
import './IncomeTable.css'; 

interface IncomeDataProps {
  rawData: {
    [country_key: string]: { 
      income: number;
      rate: number;
      amount: number;
    }
  } | null | undefined;

  income: number;
}

const IncomeData: React.FC<IncomeDataProps> = ({ rawData, income }) => {

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

  if (!rawData) {
    return null;
  }

  if (rawData === undefined) {
    return null;
  }

  const dataArray = Object.entries(rawData).map(([country, taxInfo]) => ({ country, ...taxInfo }));

  dataArray.sort((a, b) => a.rate - b.rate);

  const rankedDataArray = dataArray.map((entry, index) => ({ ...entry, rank: index + 1 }));

  const tableRows = rankedDataArray.map((entry) => (
    <tr key={entry.country}>
      <td>{entry.country}</td>
      <td>{entry.rank}</td>
      <td>{entry.rate.toFixed(2)}</td>
      <td>{entry.amount.toFixed(0)}</td>
      <td>{(income - entry.amount).toFixed(0)}</td>
    </tr>
  ));

  return (
    <div>
         <button onClick={toggleFold}>
        {isFolded ? `Show taxation by country when income is ${income}` : `Hide taxation by country when local currency income is ${income}`}
      </button>

    {!isFolded && (

    <div className="income-data-container">
      <table ref={tableRef} className="income-data-table">
        <thead>
        <tr>
                <th colSpan={5}>{`Taxation when income is ${income} in local currency`}:</th>
              </tr>
          <tr>
            <th>Country</th>
            <th>Rank</th>
            <th>Taxation %</th>
            <th>Taxation Amount (local currency)</th>
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
