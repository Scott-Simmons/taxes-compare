import './TaxTable.css';
import React, { useState, useRef, useEffect} from 'react';

interface TaxDataProps {
    rawData: {
      [country_key: string]: Array<[number, number, number | null]>;
    } | null | undefined;
  }


const TaxData: React.FC<TaxDataProps> = ({rawData}) => {
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


    return (
        <div>
          <button onClick={toggleFold}>
            {isFolded ? 'Show tax brackets (in local currency)' : 'Hide tax brackets (in local currency)'}
          </button>
    
          {!isFolded && (
            <div className="tax-data-container">
              {Object.entries(rawData).map(([country, data]) => (
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
                          {data.map(([rate, from, to], index) => (
                            <tr key={index}>
                              <td>{(+rate.toFixed(5)*100).toFixed(0)}</td>
                              <td>{from}</td>
                              <td>{to !== null ? to : '∞'}</td>
                            </tr>
                          ))}
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
