import './TaxTable.css';
import React, { useState, useRef, useEffect} from 'react';
import { BackEndResponse } from './types';

interface TaxDataProps {
    data: BackEndResponse;
  }

const TaxData: React.FC<TaxDataProps> = ({ data }) => {
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

    return (
        <div>
          <button onClick={toggleFold}>
            {isFolded ? `Show tax brackets (local currency)` : `Hide tax brackets (local currency)`}
          </button>
    
          {!isFolded && (
            <div className="tax-data-container">
              {Object.entries(data.country_specific_data).map(([country, tax_data]) => (
                <div key={country} className="country-container">
                  <div className="sticky-header-container">
                    <div className="country-name">{country}</div>
                    <div className="table-wrapper">
                      <table ref={tableRef} className="tax-table">
                        <thead>
                          <tr>
                            <th>Rate (%)</th>
                            <th>Over</th>
                            <th>Not Over</th>
                          </tr>
                        </thead>
                        <tbody>
                          {tax_data.tax_brackets.map((bracket, index) => {
                          const from = index > 0 ? tax_data.tax_brackets[index - 1].income_limit: 0;
                          const to = bracket.income_limit !== null ? bracket.income_limit : '∞';
                          return (
                            <tr key={index}>
                              <td>{(+bracket.marginal_rate.toFixed(5)*100).toFixed(0)}</td>
                              <td>{from}</td>
                              <td>{to}</td>
                            </tr>
                          )})}
                        </tbody>
                      </table>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      );
    };
    
    
    
export default TaxData;
