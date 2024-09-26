import React, {useState, useRef, useEffect} from 'react';
import { BackEndResponse } from './types';

interface ExchangeRateDataProps {
  data: BackEndResponse;
  currency: string | null;
}

const ExchangeRateTable: React.FC<ExchangeRateDataProps> = ({ data, currency }) => {

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

    if (!currency) {
     return null;
    }

  const dataArray = Object.entries(data.country_specific_data).map(([country, taxInfo]) => ({ country, taxInfo }));



  // Will not be null if currency is null, but need null handling to keep ts happy
  dataArray.sort((a, b) => (a.taxInfo.exchange_rate || 0) - (b.taxInfo.exchange_rate || 0));

  const rankedDataArray = dataArray.map((entry, index) => ({...entry, rank: index + 1}))

  // Handles edge case where update happens without needing to recompute everything
  if (rankedDataArray.every(x => x.taxInfo.exchange_rate === null)) {
    return null
  }

  const tableRows = rankedDataArray.map((entry) => (
    <tr key={entry.country}>
      <td>{entry.country}</td>
      <td>{entry.taxInfo.exchange_rate || 1.0}</td>
    </tr>
  ));

  return (
    <div>
         <button onClick={toggleFold}>
        {isFolded ? `Show exchange rates by country against ${currency}` : `Hide exchange rates by country  against ${currency}`}
      </button>

    {!isFolded && (

    <div className="exchange-rates-data-container">
      <table ref={tableRef} className="income-data-table">
        <thead>
        <tr>
          <th colSpan={5}>{`Exchange rates for (${currency})`}:</th>
        </tr>
        <tr>
          <th>Country</th>
          <th>Exchange Rate (local per {currency})</th>
        </tr>
        </thead>
        <tbody>{tableRows}</tbody>
      </table>
    </div>
  )}
  </div>
  )
};

export default ExchangeRateTable;
