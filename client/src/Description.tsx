import React from 'react';
import './Description.css';


// Not rendered right now because it looks ugly. TODO: put in later

interface ColumnProps {
    title: string;
    content: string[];
}


const Column: React.FC<ColumnProps> = ({ title, content}) => (
    <div className="Description">
        <h3>
            {title}
        </h3>
            {content.map((item, _) => (
        <p>
                {item}
                <br/>
                <br/>
        </p>
            ))}
    </div>
);

const col1text: string[] = [
    "Comparing taxes between two or more countries.", 
    "Understanding how tax varies with income in progressive taxation systems.",
]
const col2text: string[] = [
    "Tax brackets last updated on January 6th 2024.",
    "Custom user-inputted tax brackets may be added in a future release",
    "In a future update, backend will be reimplemented in Scala to support higher granularity in finding breakeven points while maintaining app performance.",
    "In a future update, option to convert plot to a specific currency will be added."
]
const col3text: string[] = [
    "Do not draw any strong conclusions without also taking into account exchange rates, income distributions, and cost of living distributions when comparing two or more countries.",
    "For faster load times, the step size increases with income, leading to potential numerical errors when computing breakeven points.", 
    "Some countries have different tax brackets dependent on martital status. This tool uses single household brackets"
]

const ThreeColumns: React.FC = () => {
    return (
        <div className='three-columns'>
            <Column title="Use this for" content={col1text}/>
            <Column title="Current/future state" content={col2text}/>
            <Column title="Disclaimers" content={col3text}/>
        </div>
    );
};

export default ThreeColumns;
