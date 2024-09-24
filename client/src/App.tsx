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
import BreakevenData from './BreakevenTable';

// TODO: This should be passed in as an env var
//const REACT_APP_API_URL: string = "http://localhost:8000";
const REACT_APP_API_URL: string = "https://taxes-compare.com";

interface BackEndResponse {

  plot_rates: {
    data: any[];
    layout: any;
  };

  plot_amounts: {
    data: any[];
    layout: any;
  };

  income_dict: {
    [country_key: string]: {
      income: number;
      rate: number;
      amount: number;
    };
  } | null | undefined;

  breakeven_dict: {
    [country_combination_key: string]: { 
      breakeven_income: number;
      breakeven_rate: number;
      breakeven_amount: number;
  }[];
  } | null | undefined;

  brackets_dict: {
    [country_key: string]: Array<[number, number, number]>
  } | null | undefined;

};

const App: React.FC = () => {
  const max_allowable_income: number = 1e9;
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
  });
  const [currency, setCurrency] = useState<string>("Local Currency");
 const handleAddCountry = (country: string) => {
    setCountries((prev) => [...prev, country]);
  };

  const handleRemoveCountry = (country: string) => {
    setCountries((prev) => prev.filter(c => c !== country));
  };
  const handleSelectCurrency = (newCurrency: string) => {
    setCurrency(newCurrency);
  };
  const handleGlobalOptionsChange = (options: Partial<GlobalOptions>) => {
    const updatedOptions = { ...globalOptions, ...options };
    if (updatedOptions.income !== undefined && (updatedOptions.income === "" || updatedOptions.income <= updatedOptions.max_income)) {
      setGlobalOptions(updatedOptions);
      setIncomeError(null);
      setMaxIncomeError(null)
    } else {
      setIncomeError(
        `Income (currently set to: ${updatedOptions.income}) must be less than the maximum income (currently set to ${updatedOptions.max_income})`
      );
    }
    if (updatedOptions.max_income === undefined || updatedOptions.max_income === 0) {
      setMaxIncomeError(`Max income (currently set to: ${updatedOptions.max_income}) gives the size of the x axis and must be greater than 0 (reccomended to set to something between 100k - 1 mil)`)
    }
    const max_income_num: number = +updatedOptions.max_income;
    if (max_income_num > max_allowable_income) {
      setMaxIncomeError(`Max income (currently set to: ${max_income_num}) is too large. Must be less than 1*10^7)`)
    }
    setGlobalOptions(updatedOptions);
  };
  const handleCompute = async () => {
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
      globalOptions: { ...globalOptions, income: globalOptions.income === '' ? 0 : globalOptions.income},
    };
    const hostname: string = REACT_APP_API_URL;
    if (!hostname) {
      return
    }
    const backendEndpoint: string = `${hostname}/process`;
    try {
      setLoading(true);
      const responseData = await axios.post(backendEndpoint, requestData, {
        headers: {
          'Content-Type': 'application/json'
        }
      });
      setresponseData(responseData.data);
      } catch (error) {
    } finally {
      setLoading(false)
    }
  };
  useEffect(() => {
    if (responseData && plotElementRef.current) {
      plotElementRef.current?.scrollIntoView({behavior: 'smooth'});
    }
  }, [responseData]);
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Enter') {
        handleCompute();
      }
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
          <RingLoader cssOverride={{display: 'flex', justifyContent: 'center', alignItems: 'center'}} size={150} color={"#36D7B7"} loading={loading} />
        </div>
      ) : (
        <>
      {responseData && (
            <div style={{ marginTop: '10px'}} ref={plotElementRef}>
            <PlotSwitcher  plotAData={responseData.plot_rates.data} plotALayout={responseData.plot_rates.layout} plotBData={responseData.plot_amounts.data} plotBLayout={responseData.plot_amounts.layout}/>
          </div>
        )}
        </>
      )}
      <br></br>
      <div id="thing">
      <TaxData rawData={responseData?.brackets_dict}/>
      <IncomeData rawData={responseData?.income_dict} income={+globalOptions.income} />
      <BreakevenData rawData={responseData?.breakeven_dict} countries={countries}/>
      </div>
      </main>
    </div>
  );
};


export default App;

