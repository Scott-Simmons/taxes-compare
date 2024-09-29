import React, { useState } from "react";
import "./PlotComp.css";
import TaxAmountsPlot from "./TaxAmountsPlot";
import TaxRatesPlot from "./TaxRatesPlot";
import { BackEndResponse } from "./types";

interface PlotSwitcherProps {
  data: BackEndResponse;
  income: number;
  currency: string;
}

const PlotSwitcher: React.FC<PlotSwitcherProps> = ({ data }) => {
  const [isDollarActive, setIsDollarActive] = useState(true);

  const toggleSwitch = () => {
    setIsDollarActive((prev) => !prev);
  };

  const switchContainerStyle = {
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    marginBottom: "20px",
  };

  return (
    <div>
      <div className="switch-container" style={switchContainerStyle}>
        <button
          className={`switch-button ${isDollarActive ? "left active" : "right active"
          }`}
          onClick={toggleSwitch}
        >
          <span>%</span>
          <span>$</span>
        </button>
      </div>
      <div className="plot-container">
        {isDollarActive ? (<TaxRatesPlot data={data}/>) : (<TaxAmountsPlot data={data}/>)}
      </div>
    </div>
  );
};

export default PlotSwitcher;
