// CustomTaxBracketPopup.tsx
import React, { useState } from 'react';

export interface CustomTaxBracket {
  rate: number;
  lowerBound: number;
  upperBound?: number;
}

interface CustomTaxBracketPopupProps {
  onClose: () => void;
  onAddBracket: (bracket: CustomTaxBracket) => void;
}

const CustomTaxBracketPopup: React.FC<CustomTaxBracketPopupProps> = ({ onClose, onAddBracket }) => {
  const [rate, setRate] = useState<number>(0);
  const [lowerBound, setLowerBound] = useState<number>(0);
  const [upperBound, setUpperBound] = useState<number | undefined>(undefined);

  const handleAddBracket = () => {
    // Validate input before adding the bracket
    if (rate >= 0 && lowerBound >= 0 && (upperBound === undefined || upperBound > lowerBound)) {
      onAddBracket({ rate, lowerBound, upperBound });
      onClose();
    } else {
      console.error('Invalid input for custom tax bracket');
    }
  };


  return (
    <div className="custom-tax-bracket-popup">
      <h3>Add Custom Tax Bracket</h3>
      <label htmlFor="rate">Rate:</label>
      <input
        type="number"
        id="rate"
        value={rate}
        onChange={(e) => setRate(Number(e.target.value))}
      />
      <label htmlFor="lowerBound">Lower Bound:</label>
      <input
        type="number"
        id="lowerBound"
        value={lowerBound}
        onChange={(e) => setLowerBound(Number(e.target.value))}
      />
      <label htmlFor="upperBound">Upper Bound (leave empty for no upper limit):</label>
      <input
        type="number"
        id="upperBound"
        value={upperBound || ''}
        onChange={(e) => setUpperBound(e.target.value === '' ? undefined : Number(e.target.value))}
      />
      <button onClick={handleAddBracket}>Add Bracket</button>
      <button onClick={onClose}>Cancel</button>
    </div>
  );
};

export default CustomTaxBracketPopup;
