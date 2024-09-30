import React, { useState, useEffect } from 'react';
import './CurrencyForm.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faInfoCircle } from '@fortawesome/free-solid-svg-icons';

interface CurrencyFormProps {
  onSelectCurrency: (currency: string | null) => void;
  currency: string | null;
}

const CurrencyForm: React.FC<CurrencyFormProps> = ({
  onSelectCurrency,
  currency,
}) => {
  const [selectedCurrency, setSelectedCurrency] = useState<string | null>(currency);
  
  const currencyOptions = [
    { value: 'Local Currency', label: 'Local Currency' },
    { value: 'AUD', label: 'Australian Dollar (AUD)' },
    { value: 'NZD', label: 'New Zealand Dollar (NZD)' },
    { value: 'CAD', label: 'Canadian Dollar (CAD)' },
    { value: 'EUR', label: 'Euro (EUR)' },
    { value: 'GBP', label: 'British Pound (GBP)' },
    { value: 'USD', label: 'US Dollar (USD)' },
    { value: 'SGD', label: 'Singapore Dollar (SGD)' },
    { value: 'ZAR', label: 'South African Rand (ZAR)' },
    { value: 'NOK', label: 'Norwegian Krone (NOK)' },
  ];

  useEffect(() => {
    setSelectedCurrency(currency);
  }, [currency]);

  const handleCurrencyChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const newCurrency = event.target.value === "Local Currency" ? null : event.target.value;
    setSelectedCurrency(newCurrency);
    onSelectCurrency(newCurrency);
  };

  const tooltipText: string = "Select your currency."

  return (
    <div className="form-container">
      <div>
        <label htmlFor="currency" className="label-text-custom">
          Select Currency:
          <span className="info-icon" data-tooltip={tooltipText}>
            <FontAwesomeIcon icon={faInfoCircle} />
          </span>
        </label>
      </div>
      <select
        id="currency"
        value={selectedCurrency === null ? "Local Currency" : selectedCurrency}
        onChange={handleCurrencyChange}
        className="select"
      >
        {currencyOptions.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </div>
  );
};

export default CurrencyForm;
