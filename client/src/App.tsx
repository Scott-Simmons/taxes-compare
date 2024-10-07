import React, { useState, useEffect, useRef} from 'react';
import './App.css';
import Header from './Header';
import CountryForm from './AddCountry';
import { GlobalOptions } from './OtherOptions';
import CurrencyForm from './CurrencyForm'
import axios from 'axios';
import GlobalOptionsForm from './OtherOptions';
import PlotSwitcher from './PlotComp';
import { RingLoader } from "react-spinners";
import TaxData from './TaxTable';
import IncomeData from './IncomeTable';
import ExchangeRateData from './ExchangeRateTable';
import BreakevenData from './BreakevenTable';
import { BackEndResponse } from './types';
import { useCallback } from 'react'

const BACKEND_API_URL: string = `${window._env_.REACT_APP_BACKEND_PROTOCOL}://${window._env_.REACT_APP_BACKEND_HOST}`;

const App: React.FC = () => {

  const max_allowable_income: number = 100e6;

  const [loading, setLoading] = useState(false);
  const [incomeError, setIncomeError] = useState<string | null>(null);
  const [maxIncomeError, setMaxIncomeError] = useState<string | null>(null);
  const [countries, setCountries] = useState<string[]>([]);
  const [responseData, setresponseData] = useState<BackEndResponse | null>(null);
  const plotElementRef = useRef<HTMLDivElement | null>(null);
  const [globalOptions, setGlobalOptions] = useState<GlobalOptions>({
    income: 0,
    showBreakevenPoints: false,
    max_income: 500000,
    countries: countries,
  });
  const [currency, setCurrency] = useState<string | null>(null);


  const handleAddCountry = (country: string) => {
    setCountries((prev) => {
      const updatedCountries = [...prev, country];
      setGlobalOptions((prevOptions) => ({
        ...prevOptions,
        countries: updatedCountries,
      }));
      return updatedCountries;
    });
  };

  const handleRemoveCountry = (country: string) => {
    setCountries((prev) => {
      const updatedCountries = prev.filter((c) => c !== country);
      setGlobalOptions((prevOptions) => ({
        ...prevOptions,
        countries: updatedCountries,
      }));
      return updatedCountries;
    });
  };

  const handleSelectCurrency = (newCurrency: string | null) => {
    setCurrency(newCurrency);
  };

  const handleGlobalOptionsChange = (options: Partial<GlobalOptions>) => {

    const updatedOptions = { ...globalOptions, ...options };

    if (updatedOptions.income !== undefined && (updatedOptions.income === null || updatedOptions.income <= updatedOptions.max_income)) {
      setGlobalOptions(updatedOptions);
      setIncomeError(null);
      setMaxIncomeError(null)
    } else {
      setIncomeError(
        `Income (currently set to: ${updatedOptions.income}) must be less than the maximum income (currently set to ${updatedOptions.max_income})`
      );
    }

    if (updatedOptions.max_income === undefined || updatedOptions.max_income === 0) {
      setMaxIncomeError(`Max income (currently set to: ${updatedOptions.max_income}) must be greater than 0`)
    }

    const max_income_num: number = +updatedOptions.max_income;

    if (max_income_num > max_allowable_income) {
      setMaxIncomeError(`Max income (currently set to: ${max_income_num}) is too large. Must be less than ${max_allowable_income})`)
    }

    setGlobalOptions(updatedOptions);

  };

  // Handle the compute, doing validation
  const handleCompute = useCallback(async () => {

    setLoading(true)

    const { income, max_income } = globalOptions;

    if (max_income === 0) {
      setLoading(false);
      alert(`Max income should be greater than 0`);
      return;
    }

    if (income > max_income) {
      setLoading(false);
      alert(`Income cannot exceed ${max_income}`);
      return;
    }

    if (+max_income > max_allowable_income) {
      setLoading(false);
      alert(`Max income cannot be greater than ${max_allowable_income}`);
      return;
    }

    if (countries.length === 0) {
      setLoading(false);
      alert("Add at least one country before computing taxes")
      return;
    }

    const requestData = {
      countries: countries,
      income: globalOptions.income === null ? 0 : globalOptions.income,
      max_income: globalOptions.max_income === null ? max_income : globalOptions.max_income,
      show_break_even: globalOptions.showBreakevenPoints,
      normalizing_currency: currency
    };

    const hostname: string = BACKEND_API_URL;
    if (!hostname) {
      return
    }

    const backendEndpoint: string = `${hostname}/process`;

    try {
      setLoading(true);
      const responseData = await axios.post(backendEndpoint, requestData, {
        headers: {
          'Content-Type': 'application/json'
      }});
      setresponseData(responseData.data);
      } catch {
        throw new Error(`Issue with request: ${backendEndpoint}, ${JSON.stringify(requestData)}`)
    } finally {
      setLoading(false)
    }
  }, [countries, currency, globalOptions]);

  // Effects
  useEffect(() => {
    if (responseData && plotElementRef.current) {
      plotElementRef.current?.scrollIntoView({behavior: 'smooth'});
    }
  }, [responseData]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Enter') {handleCompute();}
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [handleCompute]);


  return (
    <div className="App">
        <Header />
      <main>
      <CountryForm 
        onAddCountry={handleAddCountry}
        onRemoveCountry={handleRemoveCountry}
        countries={countries}
      />
      <CurrencyForm
        currency={currency}
        onSelectCurrency={handleSelectCurrency}
      />
      <GlobalOptionsForm
        globalOptions={globalOptions}
        onGlobalOptionsChange={handleGlobalOptionsChange}
        incomeError={incomeError}
        maxIncomeError={maxIncomeError}
        onComputeButtonClick={handleCompute}
      />
      {loading ? (
        <div className="loading-container">
          <RingLoader cssOverride={{display: 'flex', justifyContent: 'center', alignItems: 'center'}} size={150} color={"#36D7B7"} loading={loading}/>
        </div>
      ) : (
        <>
      {responseData && (
            <div style={{ marginTop: '10px'}} ref={plotElementRef}>
            <PlotSwitcher data={responseData} income={globalOptions.income} currency={currency || "Local Currency"}/>
            </div>
        )}
        </>
      )}
      <br></br>
      <div id="brackettables">
       {responseData && (
        <div style={{ marginTop: '10px' }}>
          <TaxData data={responseData}/>
        </div>
        ) 
        }
      </div>
      <div id="incometables">
       {responseData && (
        <div style={{ marginTop: '10px' }}>
          <IncomeData income={globalOptions.income} data={responseData} currency={currency}/>
        </div>
        )
        }
      </div>
      <div id="breakeventables">
       {responseData && (
        <div style={{ marginTop: '10px' }}>
          <BreakevenData data={responseData} currency={currency}/>
        </div>
        )
        }
      </div>
      <div id="exchangeratetables">
       {responseData && (
        <div style={{ marginTop: '10px' }}>
          <ExchangeRateData data={responseData}/>
        </div>
        ) 
        }
      </div>
      </main>
    </div>
  );
};

export default App;
