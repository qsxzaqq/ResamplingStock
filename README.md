# ResamplingStock
A股重采样 ClickHouse UDF

## 支持范围
- 5min
- 15min

## 食用方式:
```
SELECT symbol, groupArray(`datetime`)[1] AS datetime_new, argMin(`open`, `datetime`) AS open_new, max(`high`) AS high_new, min(`low`) AS low_new, argMax(`close`, `datetime`) AS close_new, sum(`total_turnover`) AS total_turnover_new, total_turnover_new / close_new as volume_new FROM quant.stock_1min WHERE `datetime` BETWEEN '2022-06-01 00:00:00' AND '2022-06-01 23:59:59' GROUP BY `symbol`, resampling_stock(`datetime`, '5min')
```