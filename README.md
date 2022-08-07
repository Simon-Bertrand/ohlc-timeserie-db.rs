
-Rust Timeserie Database<br /><br />

<p>This project aims only on a specific need for a <a href="https://en.wikipedia.org/wiki/Open-high-low-close_chart" title="OHCL"> OHCL data </a> storage.
It focuses on multidimensionnal data sampled with a specific frequency, where each index is ordered. The system will have rules that user who wants to push data cannot  break. </p><br />

<p>The sample frequency will be called <b>Fe</b> so the time step between each sample will be <b>Te</b></p>
Samples are defined as a struct : <br />
<ul>
            <li>ts: i64 - timestamp (s) of current sample</li>
            <li>o : Option(Decimal) - open price <i>(random reference asset / random asset. Ex: EUR/USD)</i></li>
            <li>h : Option(Decimal) - high price</li>
            <li>c : Option(Decimal) - close price</li>
            <li>l : Option(Decimal) - low price</li>
</ul>


<h3>Rules</h3>
- Defined null sample : A null sample can sees its data null but its timestamp cannot be undefined. The null state for a sample is defined without an empty timestamp.<br />
- Ts Ordered  : Each inserted sample n2 after a sample n1, even if null, has to verify the fact ts2-ts1 = DEFAULT_STEP. Where DEFAULT_STEP is equal to <b>Te</b>.<br />
- Blocs splitted data : Each binary block has a maximum number of write samples. Once exceeded, a new bloc is created.<br />
- Non-zero Ts : A collection cannot start with a zero timestamp as this value is used for the initialisation process.<br />
- Indexing utils : A bloc size has to be a multiple of the batch size.
<h3>Encoding / Compression</h3>

Native Rust only manages bytes, so we can't manipulate bits without a library like bitvec or something else. In order to make a high space efficient encoding, few utf_8 characters are choosen to be encoded and decoded in a specific bytes format. The number of these characters is 14, the following list : 
<ul>
            <li>"0-9", numeric numbers bytes (n=10)</li> 
            <li>".", numeric dot byte (n=11)</li> 
            <li>"*", sample start byte (n=12)</li> 
            <li>";", separator byte (n=13)</li>
            <li>"-", null byte (n=14)</li> 
</ul>
Normally, we would need only 4 bits to encode all these characters but, Native Rust restrictions, we will use a full byte to encode 2 characters considering the fact that the four first bytes for our encoding map are always zero.

<h3>Indexing</h3>

Indexing is used with sparse indexes. We split the same timeserie into multiple blocs (bigger) or batchs (smaller). Bloc size has to be a multiple of the batch size. At each start of batch, we store in the index map the start byte position in the bloc file corresponding to the start batch. As our data is ordered by timestamp, this reduce a lot the storage space used by the index map. We can easily open a file with a cursor at the given position to take a specific number of sample.<br/><br/>
The index map is stored and encoded using the Bincode format to avoid Rust types parsing when readed from a file. The positions and their corresponding batch ids are stored in a BTreeMap to gain efficiency.

<h3>Query string</h3>

The query string is a command which allows users to fastly ask for a specific batch of data or a timestamp range. We use two kind of command : <br />
<ul>
            <li>
                       <b>By timestamp : </b>ts source:collection::ts_start:ts_end
            </li>
            <li>
                       <b>By aggregated timestamp : </b>ts -([a-zA-Z]+) source:collection::ts_start:ts_end
            </li>
            <li>
                        <b>By batch : </b>batch source:collection::batchid
            </li>
</ul> 
For the timestamp command, we floor the start ts and ceil the end ts to fit with the different batch positions.
Aggregation is available for the OHLC aggregation function.

