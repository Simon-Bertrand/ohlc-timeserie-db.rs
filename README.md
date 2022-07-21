
-Rust Timeserie Database<br /><br />

<p>This project aims only on a specific need for a <a href="https://en.wikipedia.org/wiki/Open-high-low-close_chart" title="OHCL"> OHCL data </a> storage.
It focuses on multidimensionnal data sampled with a specific frequency, where each index is ordered. The system will have rules that user who wants to push data cannot  break. </p><br />

<p>The sample frequency will be called <b>fe</b> so the time step between each sample will be <b>te</b></p>
Samples are defined as a struct : <br />
<ul>
            <li>ts: i64 - timestamp (s) of current sample</li>
            <li>o : Option<Decimal> - open price <i>(random reference asset / random asset. Ex: EUR/USD)</i></li>
            <li>h : Option<Decimal> - high price</li>
            <li>c : Option<Decimal> - close price</li>
            <li>l : Option<Decimal> - low price</li>
</ul>


<h3>Rules</h3>
-A null sample can sees its data null but its timestamp cannot be undefined. The null state for a sample is defined without an empty timestamp.
-Each inserted sample n2 after a sample n1, even if null, has to verify the fact ts2-ts1 = DEFAULT_STEP. Where DEFAULT_STEP is equal to <b>te</b>.
-Each binary block has a maximum number of write samples. Once exceeded, a new bloc is created.