import React, { useState } from "react";
import Plot from "react-plotly.js";
import "./PlotComp.css";

interface PlotComponentProps {
  // TODO: Get actual types lol
  plotAData: any[]; 
  plotALayout: any; 
  plotBData: any[];
  plotBLayout: any;
}

const PlotSwitcher: React.FC<PlotComponentProps> = ({
  plotAData,
  plotALayout,
  plotBData,
  plotBLayout,
}) => {
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
        {isDollarActive ? (
          <Plot
            data={plotAData}
            layout={plotALayout}
            useResizeHandler={true}
            style={{ width: "80vw", height: "80vh" }}
          />
        ) : (
          <Plot
            data={plotBData}
            layout={plotBLayout}
            useResizeHandler={true}
            style={{ width: "80vw", height: "80vh" }}
          />
        )}
      </div>
    </div>
  );
};

export default PlotSwitcher;
