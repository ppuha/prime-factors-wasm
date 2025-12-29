import { h, render } from "https://esm.sh/preact";
import { useState } from "https://esm.sh/preact/hooks";
import htm from "https://esm.sh/htm";
import init from "./lib/factors.js";
import { factors } from "./lib/factors.js";

const e = htm.bind(h);

await init();

let num = 100;
let fs = factors(num);

const Factors = ({ num, factors }) => e`
  <p>${num} = ${factors.map((f) => f.to_string()).join(" * ")}</p>
`;

const Input = ({ setNum, setFs }) => e`
  <input
    type=number
    onChange=${(e) => {
      let num = e.target.value;
      let fs = factors(num);
      setNum(num);
      setFs(fs);
    }}/>
`;

const App = ({}) => {
  let [num, setNum] = useState(0);
  let [fs, setFs] = useState([]);

  return e`
    <div>
      <${Input} setNum=${setNum} setFs=${setFs}/>
      <${Factors} num=${num} factors=${fs} />
    </div>
  `;
};

let root = document.getElementById("root");
render(e`<${App}/>`, root);
