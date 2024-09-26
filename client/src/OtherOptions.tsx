import React, {useState} from 'react';
import './OtherOptions.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faInfoCircle } from '@fortawesome/free-solid-svg-icons';

export interface GlobalOptions {
  income: number;
  showBreakevenPoints: boolean;
  max_income: number | string;
  countries: string[];
}

interface GlobalOptionsFormProps {
  globalOptions: GlobalOptions;
  onGlobalOptionsChange: (options: Partial<GlobalOptions>) => void;
  incomeError: string | null;
  maxIncomeError: string | null;
  onComputeButtonClick: any;
}

const GlobalOptionsForm: React.FC<GlobalOptionsFormProps> = ({
  globalOptions,
  onGlobalOptionsChange,
  incomeError,
  maxIncomeError,
  onComputeButtonClick,
}) => {
  const [incomeInfoVisible, _] = useState(false);

  return (
    <div className="global-options-form">

      <div className="form-group">
        <label className="label-text-custom" htmlFor="income">
          Income: (optional)
          <span className="info-icon" data-tooltip="Add specific income to analyze">
            <FontAwesomeIcon icon={faInfoCircle} />
          </span>
          {incomeInfoVisible && (
            <div className="info-tooltip">
              Additional information about the income input.
            </div>
          )}
        </label>
        <input
          className="input-field"
          type="tel"
          id="income"
          defaultValue={globalOptions.income === 0 ? '' : globalOptions.income}
          onChange={(e) => onGlobalOptionsChange({ income: Number(e.target.value) })}
        />
        {incomeError && <div className="error-message">{incomeError}</div>}
      </div>

      <div className="form-group">
        <label className="label-text-custom" htmlFor="max_income">
          Maximum Income to consider: (scale)
          <span className="info-icon" data-tooltip="Set the max value for the x axis.">
            <FontAwesomeIcon icon={faInfoCircle} />
          </span>
        </label>
        <input
          className="input-field"
          type="tel"
          id="max_income"
          defaultValue={globalOptions.max_income}
          onChange={(e) => onGlobalOptionsChange({ max_income: Number(e.target.value) })}
        />
        {maxIncomeError && <div className="error-message">{maxIncomeError}</div>}
      </div>

      {globalOptions.countries.length > 1 && (<div className={'form-group checkbox-group'}>
      <label className="label-text-custom checkbox-label" htmlFor="showBreakevenPoints">
        <input
          className="checkbox-field"
          type="checkbox"
          id="showBreakevenPoints"
          checked={globalOptions.showBreakevenPoints}
          onChange={(e) => onGlobalOptionsChange({ showBreakevenPoints: e.target.checked })}
        />
        Show Breakeven Points
        <span className="info-icon" data-tooltip="Additional info about breakeven points">
          <FontAwesomeIcon icon={faInfoCircle} />
        </span>
      </label>
      </div>)}

      <div className="form-group">
      <button className='compute-button' onClick={onComputeButtonClick}>
        Compute Taxes
      </button>
      </div>

    </div>
  );
};

export default GlobalOptionsForm;
