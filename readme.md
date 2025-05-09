<h1>Math</h1>
<h2>A library..</h2>

<p1>
    Although floating-point numbers are widely used, fixed-point values offer super consistency and safety. Fixed-point numbers do not suffer from ambiguous standards ensuring that numeric data is reliably transferred and decoded in the correct format.
</p1>


<h6>Symbols</h6>
<div
    style="
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        gap: 50px;
        min-width: 100%;
    ">
    <div>
        <table>
            <tr>
                <th>
                    Symbol
                </th>
                <th></th>
                <th></th>
            </tr>
            <tr>
                <td>K</td>
                <td>1,000</td>
                <td>THOUSAND</td>
            </tr>
            <tr>
                <td>M</td>
                <td>1,000,000</td>
                <td>MILLION</td>
            </tr>
            <tr>
                <td>B</td>
                <td>1,000,000,000</td>
                <td>BILLION</td>
            </tr>
            <tr>
                <td>T</td>
                <td>1,000,000,000,000</td>
                <td>TRILLION</td>
            </tr>
        </table>
    </div>
    <div>
        <table>
            <tr>
                <th>Type</th>
                <th>Range</th>
            </tr>
            <tr>
                <td>u8</td>
                <td>~255</td>
            </tr>
            <tr>
                <td>u16</td>
                <td>K ~65</td>
            </tr>
            <tr>
                <td>u32</td>
                <td>B ~4</td>
            </tr>
            <tr></tr>
        </table>
    </div>
</div>



###### Symbol Legend
| Symbol |                                           |          |
|--------|-------------------------------------------|----------|
| K      | 1,000                                     | THOUSAND |
| M      | 1,000,000                                 | MILLION  |
| B      | 1,000,000,000                             | BILLION  |
| T      | 1,000,000,000,000                         | TRILLION |
| P      | 1,000,000,000,000,000                     | PETA     |
| E      | 1,000,000,000,000,000,000                 | EXA      |
| Z      | 1,000,000,000,000,000,000,000             | ZETTA    |
| Y      | 1,000,000,000,000,000,000,000,000         | YOTTA    |
| R      | 1,000,000,000,000,000,000,000,000,000     | RONNA    |
| Q      | 1,000,000,000,000,000,000,000,000,000,000 | QUETTA   |

###### Native Primitive Cap
| Type | Max          | Min           | Size |
|------|--------------|---------------|------|
| u8   | ~255         | 0             | 1    |
| u16  | K ~65        | 0             | 2    |
| u32  | B ~4         | 0             | 4    |
| u64  | E ~18        | 0             | 8    |
| u128 | Q ~340000000 | 0             | 16   |
| i8   | ~127         | ~-128         | 1    |
| i16  | K ~32        | K ~-32        | 2    |
| i32  | B ~2         | B ~-2         | 4    |
| i64  | E ~9         | E ~-9         | 8    |
| i128 | Q ~170000000 | Q ~-170000000 | 16   |



<h5>Q Variant Sheet</h5>
<p2>
    Types with values up to the 38th precision are representable. Representable values are rounded down, so the actual value exceeds what is listed here. Types like unsigned integers cannot represent negative values.
</p2>
<div
    style="
        display: flex;
        flex-wrap: wrap;
        flex-direction: row;
        justify-content: center;
        align-items: start;
        gap: 50px; 
        min-width: 100%;
    ">
    <div>
        <h6>U8</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1U8</td>
                <td>1</td>
                <td>~25</td>
            </tr>
            <tr>
                <td>Q2U8</td>
                <td>2</td>
                <td>~2</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>U16</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1U16</td>
                <td>1</td>
                <td>K ~6</td>
            </tr>
            <tr>
                <td>Q2U16</td>
                <td>2</td>
                <td>~655</td>
            </tr>
            <tr>
                <td>Q3U16</td>
                <td>3</td>
                <td>~65</td>
            </tr>
            <tr>
                <td>Q4U16</td>
                <td>4</td>
                <td>~6</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>U32</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1U32</td>
                <td>1</td>
                <td>M ~429</td>
            </tr>
            <tr>
                <td>Q2U32</td>
                <td>2</td>
                <td>M ~42</td>
            </tr>
            <tr>
                <td>Q3U32</td>
                <td>3</td>
                <td>M ~3</td>
            </tr>
            <tr>
                <td>Q4U32</td>
                <td>4</td>
                <td>K ~429</td>
            </tr>
            <tr>
                <td>Q5U32</td>
                <td>5</td>
                <td>K ~42</td>
            </tr>
            <tr>
                <td>Q6U32</td>
                <td>6</td>
                <td>K ~4</td>
            </tr>
            <tr>
                <td>Q7U32</td>
                <td>7</td>
                <td>~429</td>
            </tr>
            <tr>
                <td>Q8U32</td>
                <td>8</td>
                <td>~42</td>
            </tr>
            <tr>
                <td>Q9U32</td>
                <td>7</td>
                <td>~4</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>U64</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1U64</td>
                <td>1</td>
                <td>E ~1</td>
            </tr>
            <tr>
                <td>Q2U64</td>
                <td>2</td>
                <td>P ~184</td>
            </tr>
            <tr>
                <td>Q3U64</td>
                <td>3</td>
                <td>P ~18</td>
            </tr>
            <tr>
                <td>Q4U64</td>
                <td>4</td>
                <td>P ~1</td>
            </tr>
            <tr>
                <td>Q5U64</td>
                <td>5</td>
                <td>T ~184</td>
            </tr>
            <tr>
                <td>Q6U64</td>
                <td>6</td>
                <td>T ~18</td>
            </tr>
            <tr>
                <td>Q7U64</td>
                <td>7</td>
                <td>T ~1</td>
            </tr>
            <tr>
                <td>Q8U64</td>
                <td>8</td>
                <td>B ~184</td>
            </tr>
            <tr>
                <td>Q9U64</td>
                <td>9</td>
                <td>B ~18</td>
            </tr>
            <tr>
                <td>Q10U64</td>
                <td>10</td>
                <td>B ~1</td>
            </tr>
            <tr>
                <td>Q11U64</td>
                <td>11</td>
                <td>M ~184</td>
            </tr>
            <tr>
                <td>Q12U64</td>
                <td>12</td>
                <td>M ~18</td>
            </tr>
            <tr>
                <td>Q13U64</td>
                <td>13</td>
                <td>M ~1</td>
            </tr>
            <tr>
                <td>Q14U64</td>
                <td>14</td>
                <td>K ~184</td>
            </tr>
            <tr>
                <td>Q15U64</td>
                <td>15</td>
                <td>K ~18</td>
            </tr>
            <tr>
                <td>Q16U64</td>
                <td>16</td>
                <td>K ~1</td>
            </tr>
            <tr>
                <td>Q17U64</td>
                <td>17</td>
                <td>~184</td>
            </tr>
            <tr>
                <td>Q18U64</td>
                <td>18</td>
                <td>~18</td>
            </tr>
            <tr>
                <td>Q19U64</td>
                <td>19</td>
                <td>~1</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>U128</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1U128</td>
                <td>1</td>
                <td>Q ~34000000</td>
            </tr>
            <tr>
                <td>Q2U128</td>
                <td>2</td>
                <td>Q ~3400000</td>
            </tr>
            <tr>
                <td>Q3U128</td>
                <td>3</td>
                <td>Q ~340000</td>
            </tr>
            <tr>
                <td>Q4U128</td>
                <td>4</td>
                <td>Q ~34000</td>
            </tr>
            <tr>
                <td>Q5U128</td>
                <td>5</td>
                <td>Q ~3400</td>
            </tr>
            <tr>
                <td>Q6U128</td>
                <td>6</td>
                <td>Q ~340</td>
            </tr>
            <tr>
                <td>Q7U128</td>
                <td>7</td>
                <td>Q ~34</td>
            </tr>
            <tr>
                <td>Q8U128</td>
                <td>8</td>
                <td>Q ~3</td>
            </tr>
            <tr>
                <td>Q9U128</td>
                <td>9</td>
                <td>R ~340</td>
            </tr>
            <tr>
                <td>Q10U128</td>
                <td>10</td>
                <td>R ~32</td>
            </tr>
            <tr>
                <td>Q11U128</td>
                <td>11</td>
                <td>R ~3</td>
            </tr>
            <tr>
                <td>Q12U128</td>
                <td>12</td>
                <td>Y ~340</td>
            </tr>
            <tr>
                <td>Q13U128</td>
                <td>13</td>
                <td>Y ~34</td>
            </tr>
            <tr>
                <td>Q14U128</td>
                <td>14</td>
                <td>Y ~3</td>
            </tr>
            <tr>
                <td>Q15U128</td>
                <td>15</td>
                <td>Z ~340</td>
            </tr>
            <tr>
                <td>Q16U128</td>
                <td>16</td>
                <td>Z ~34</td>
            </tr>
            <tr>
                <td>Q17U128</td>
                <td>17</td>
                <td>Z ~3</td>
            </tr>
            <tr>
                <td>Q18U128</td>
                <td>18</td>
                <td>E ~340</td>
            </tr>
            <tr>
                <td>Q19U128</td>
                <td>19</td>
                <td>E ~34</td>
            </tr>
            <tr>
                <td>Q20U128</td>
                <td>20</td>
                <td>E ~3</td>
            </tr>
            <tr>
                <td>Q21U128</td>
                <td>21</td>
                <td>P ~340</td>
            </tr>
            <tr>
                <td>Q22U128</td>
                <td>22</td>
                <td>P ~34</td>
            </tr>
            <tr>
                <td>Q23U128</td>
                <td>23</td>
                <td>P ~3</td>
            </tr>
            <tr>
                <td>Q24U128</td>
                <td>24</td>
                <td>T ~340</td>
            </tr>
            <tr>
                <td>Q25U128</td>
                <td>25</td>
                <td>T ~34</td>
            </tr>
            <tr>
                <td>Q26U128</td>
                <td>26</td>
                <td>T ~3</td>
            </tr>
            <tr>
                <td>Q27U128</td>
                <td>27</td>
                <td>B ~340</td>
            </tr>
            <tr>
                <td>Q28U128</td>
                <td>28</td>
                <td>B ~34</td>
            </tr>
            <tr>
                <td>Q29U128</td>
                <td>29</td>
                <td>B ~3</td>
            </tr>
            <tr>
                <td>Q30U128</td>
                <td>30</td>
                <td>M ~340</td>
            </tr>
            <tr>
                <td>Q31U128</td>
                <td>31</td>
                <td>M ~34</td>
            </tr>
            <tr>
                <td>Q32U128</td>
                <td>32</td>
                <td>M ~3</td>
            </tr>
            <tr>
                <td>Q33U128</td>
                <td>33</td>
                <td>K ~340</td>
            </tr>
            <tr>
                <td>Q34U128</td>
                <td>34</td>
                <td>K ~34</td>
            </tr>
            <tr>
                <td>Q35U128</td>
                <td>35</td>
                <td>K ~3</td>
            </tr>
            <tr>
                <td>Q36U128</td>
                <td>36</td>
                <td>~340</td>
            </tr>
            <tr>
                <td>Q37U128</td>
                <td>37</td>
                <td>~34</td>
            </tr>
            <tr>
                <td>Q38U128</td>
                <td>38</td>
                <td>~3</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>I8</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1I8</td>
                <td>1</td>
                <td>±~12</td>
            </tr>
            <tr>
                <td>Q2I8</td>
                <td>2</td>
                <td>±~1</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>I16</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1I16</td>
                <td>1</td>
                <td>K ±~3</td>
            </tr>
            <tr>
                <td>Q2I16</td>
                <td>2</td>
                <td>±~327</td>
            </tr>
            <tr>
                <td>Q3I16</td>
                <td>3</td>
                <td>±~32</td>
            </tr>
            <tr>
                <td>Q4I16</td>
                <td>4</td>
                <td>±~3</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>I32</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1I32</td>
                <td>1</td>
                <td>M ±~214</td>
            </tr>
            <tr>
                <td>Q2I32</td>
                <td>2</td>
                <td>M ±~21</td>
            </tr>
            <tr>
                <td>Q3I32</td>
                <td>3</td>
                <td>M ±~2</td>
            </tr>
            <tr>
                <td>Q4I32</td>
                <td>4</td>
                <td>K ±~214</td>
            </tr>
            <tr>
                <td>Q5I32</td>
                <td>5</td>
                <td>K ±~21</td>
            </tr>
            <tr>
                <td>Q6I32</td>
                <td>6</td>
                <td>K ±~2</td>
            </tr>
            <tr>
                <td>Q7I32</td>
                <td>7</td>
                <td>±~214</td>
            </tr>
            <tr>
                <td>Q8I32</td>
                <td>8</td>
                <td>±~21</td>
            </tr>
            <tr>
                <td>Q9I32</td>
                <td>9</td>
                <td>±~2</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>I64</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1I64</td>
                <td>1</td>
                <td>P ±~922</td>
            </tr>
            <tr>
                <td>Q2I64</td>
                <td>2</td>
                <td>P ±~92</td>
            </tr>
            <tr>
                <td>Q3I64</td>
                <td>3</td>
                <td>P ±~9</td>
            </tr>
            <tr>
                <td>Q4I64</td>
                <td>4</td>
                <td>T ±~922</td>
            </tr>
            <tr>
                <td>Q5I64</td>
                <td>5</td>
                <td>T ±~92</td>
            </tr>
            <tr>
                <td>Q6I64</td>
                <td>6</td>
                <td>T ±~9</td>
            </tr>
            <tr>
                <td>Q7I64</td>
                <td>7</td>
                <td>B ±~922</td>
            </tr>
            <tr>
                <td>Q8I64</td>
                <td>8</td>
                <td>B ±~92</td>
            </tr>
            <tr>
                <td>Q9I64</td>
                <td>9</td>
                <td>B ±~9</td>
            </tr>
            <tr>
                <td>Q10I64</td>
                <td>10</td>
                <td>M ±~922</td>
            </tr>
            <tr>
                <td>Q11I64</td>
                <td>11</td>
                <td>M ±~92</td>
            </tr>
            <tr>
                <td>Q12I64</td>
                <td>12</td>
                <td>M ±~9</td>
            </tr>
            <tr>
                <td>Q13I64</td>
                <td>13</td>
                <td>K ±~922</td>
            </tr>
            <tr>
                <td>Q14I64</td>
                <td>14</td>
                <td>K ±~92</td>
            </tr>
            <tr>
                <td>Q15I64</td>
                <td>15</td>
                <td>K ±~9</td>
            </tr>
            <tr>
                <td>Q16I64</td>
                <td>16</td>
                <td>±~922</td>
            </tr>
            <tr>
                <td>Q17I64</td>
                <td>17</td>
                <td>±~92</td>
            </tr>
            <tr>
                <td>Q18I64</td>
                <td>18</td>
                <td>±~9</td>
            </tr>
        </table>
    </div>
    <div>
        <h6>I128</h6>
        <table>
            <tr>
                <th>Type</th>
                <th>Precision</th>
                <th>Representable</th>
            </tr>
            <tr>
                <td>Q1I128</td>
                <td>1</td>
                <td>Q ±~17000000</td>
            </tr>
            <tr>
                <td>Q2I128</td>
                <td>2</td>
                <td>Q ±~1700000</td>
            </tr>
            <tr>
                <td>Q3I128</td>
                <td>3</td>
                <td>Q ±~170000</td>
            </tr>
            <tr>
                <td>Q4I128</td>
                <td>4</td>
                <td>Q ±~17000</td>
            </tr>
            <tr>
                <td>Q5I128</td>
                <td>5</td>
                <td>Q ±~1700</td>
            </tr>
            <tr>
                <td>Q6I128</td>
                <td>6</td>
                <td>Q ±~170</td>
            </tr>
            <tr>
                <td>Q7I128</td>
                <td>7</td>
                <td>Q ±~17</td>
            </tr>
            <tr>
                <td>Q8I128</td>
                <td>8</td>
                <td>Q ±~1</td>
            </tr>
            <tr>
                <td>Q9I128</td>
                <td>9</td>
                <td>R ±~170</td>
            </tr>
            <tr>
                <td>Q10I128</td>
                <td>10</td>
                <td>R ±~17</td>
            </tr>
            <tr>
                <td>Q11I128</td>
                <td>11</td>
                <td>R ±~1</td>
            </tr>
            <tr>
                <td>Q12I128</td>
                <td>12</td>
                <td>Y ±~170</td>
            </tr>
            <tr>
                <td>Q13I128</td>
                <td>13</td>
                <td>Y ±~17</td>
            </tr>
            <tr>
                <td>Q14I128</td>
                <td>14</td>
                <td>Y ±~1</td>
            </tr>
            <tr>
                <td>Q15I128</td>
                <td>15</td>
                <td>Z ±~170</td>
            </tr>
            <tr>
                <td>Q16I128</td>
                <td>16</td>
                <td>Z ±~17</td>
            </tr>
            <tr>
                <td>Q17I128</td>
                <td>17</td>
                <td>Z ±~1</td>
            </tr>
            <tr>
                <td>Q18I128</td>
                <td>18</td>
                <td>E ±~170</td>
            </tr>
            <tr>
                <td>Q19I128</td>
                <td>19</td>
                <td>E ±~17</td>
            </tr>
            <tr>
                <td>Q20I128</td>
                <td>20</td>
                <td>E ±~1</td>
            </tr>
            <tr>
                <td>Q21I128</td>
                <td>21</td>
                <td>P ±~170</td>
            </tr>
            <tr>
                <td>Q22I128</td>
                <td>22</td>
                <td>P ±~17</td>
            </tr>
            <tr>
                <td>Q23I128</td>
                <td>23</td>
                <td>P ±~1</td>
            </tr>
            <tr>
                <td>Q24I128</td>
                <td>24</td>
                <td>T ±~170</td>
            </tr>
            <tr>
                <td>Q25I128</td>
                <td>25</td>
                <td>T ±~17</td>
            </tr>
            <tr>
                <td>Q26I128</td>
                <td>26</td>
                <td>T ±~1</td>
            </tr>
            <tr>
                <td>Q27I128</td>
                <td>27</td>
                <td>B ±~170</td>
            </tr>
            <tr>
                <td>Q28I128</td>
                <td>28</td>
                <td>B ±~17</td>
            </tr>
            <tr>
                <td>Q29I128</td>
                <td>29</td>
                <td>B ±~1</td>
            </tr>
            <tr>
                <td>Q30I128</td>
                <td>30</td>
                <td>M ±~170</td>
            </tr>
            <tr>
                <td>Q31I128</td>
                <td>31</td>
                <td>M ±~17</td>
            </tr>
            <tr>
                <td>Q32I128</td>
                <td>32</td>
                <td>M ±~1</td>
            </tr>
            <tr>
                <td>Q33I128</td>
                <td>33</td>
                <td>K ±~170</td>
            </tr>
            <tr>
                <td>Q34I128</td>
                <td>34</td>
                <td>K ±~17</td>
            </tr>
            <tr>
                <td>Q35I128</td>
                <td>35</td>
                <td>K ±~1</td>
            </tr>
            <tr>
                <td>Q36I128</td>
                <td>36</td>
                <td>±~170</td>
            </tr>
            <tr>
                <td>Q37I128</td>
                <td>37</td>
                <td>±~17</td>
            </tr>
            <tr>
                <td>Q38I128</td>
                <td>38</td>
                <td>±~1</td>
            </tr>
        </table>
    </div>    
</div>