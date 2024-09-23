import React, { useState, useEffect } from 'react';
import './AddCountry.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faInfoCircle } from '@fortawesome/free-solid-svg-icons';

interface CountryFormProps {
  onAddCountry: (country: string) => void;
  onRemoveCountry: (country: string) => void;
  countries: string[];
}

const CountryForm: React.FC<CountryFormProps> = ({
  onAddCountry,
  countries: propCountries,
  onRemoveCountry,
}) => {
  const [selectedCountry, setSelectedCountry] = useState<string>('');
  const [countries, setCountries] = useState<string[]>(propCountries); 

  useEffect(() => {
    setCountries(Array.from(new Set(propCountries)));
  }, [propCountries]);

  const handleAddCountry = (country: string) => {
    if (country.trim() !== '' && !countries.includes(country)) {
      onAddCountry(country);
      setCountries((prevCountries) => [...prevCountries, country]);
      setSelectedCountry('');
    }
  };

  const handleRemoveCountry = (country: string) => {
    onRemoveCountry(country);
    setCountries((prevCountries) => prevCountries.filter((c) => c !== country));
  };

  const tooltipText: string = "Add one or more countries."


  return (
    <div className="form-container">
      <div>
      <label htmlFor="country" className="label-text-custom">
        {
          countries.length === 0 ?
          'Add Country:'
          : 'Add More Countries:'}
      <span className="info-icon" data-tooltip={tooltipText}>
        <FontAwesomeIcon icon={faInfoCircle} />
        </span>
      </label>
      </div>
      <select
        id="country"
        value={selectedCountry}
        onChange={(e) => {
          setSelectedCountry(e.target.value);
          handleAddCountry(e.target.value);
        }}
        className="select"
      >
        <option value="">Select a country</option>
        <option value="Australia">Australia</option>
        <option value="Canada (excl. provincial taxes)">
          Canada (excluding provincial taxes)
        </option>
        <option value="Ireland">Ireland</option>
        <option value="Netherlands">Netherlands</option>
        <option value="New Zealand">New Zealand</option>
        <option value="Norway">Norway</option>
        <option value="Singapore">Singapore</option>
        <option value="South Africa">South Africa</option>
        <option value="Spain">Spain</option>
        <option value="United Kingdom">United Kingdom</option>
        <option value="United States of America (excl. state taxes)">
          United States of America (excluding state taxes)
        </option>
      </select>

      <ul className="list">
        {countries.map((country) => (
          <li key={country} className="list-item">
            {country}{' '}
            <button
              onClick={() => handleRemoveCountry(country)}
              className="remove-button"
            >
              Remove
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
};



export default CountryForm;
