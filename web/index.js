import { h, render } from "https://esm.sh/preact";
import { useState } from "https://esm.sh/preact/hooks";
import htm from "https://esm.sh/htm";
import init from "./lib/factors.js";
import { factors } from "./lib/factors.js";

const e = htm.bind(h);

await init();

let num = 100;
let fs = factors(num);

const factorOrder = (f0, f1) => f0.prime - f1.prime;

const Factor = ({ factor }) =>
  factor.deg == 1
    ? e`
    <mo>${factor.prime}</mo>`
    : e`
    <msup>
      <mi>${factor.prime}</mi>
      <mn>${factor.deg}</mn>
      </msup>
    `;

const Result = ({ num, factors }) => e`
  <math>
    <mrow>
      <mo>${num}</mo>
      <mo> = </mo>
      ${factors.flatMap((f, i) =>
        i < factors.length - 1
          ? [e`<${Factor} factor=${f}/>`, e`<mo>*</mo>`]
          : e`<${Factor} factor=${f}/>`,
      )}
    </mrow>
  </math>
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
      <p><${Result} num=${num} factors=${fs.sort(factorOrder)}/></p>
    </div>
  `;
};

let root = document.getElementById("root");
render(e`<${App}/>`, root);
