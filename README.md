# ResamplingStock
A股重采样 ClickHouse UDF

## 支持范围
- 1min
- 5min
- 15min
- 30min
- 60min

## 食用方式:
```
SELECT symbol, date_trunc('minute', max(`datetime`)) AS datetime_new, argMin(`last`, `datetime`) AS open_new, max(`last`) AS high_new, min(`last`) AS low_new, argMax(`last`, `datetime`) AS close_new, sum(`total_turnover`) AS total_turnover_new, sum(`volume`) as volume_new FROM quant.stock_tick WHERE `datetime` BETWEEN '2022-06-14 09:30:00' AND '2022-06-14 15:00:59' GROUP BY `symbol`, resampling_stock(`datetime`, '1min')
SELECT symbol, max(`datetime`) AS datetime_new, argMin(`open`, `datetime`) AS open_new, max(`high`) AS high_new, min(`low`) AS low_new, argMax(`close`, `datetime`) AS close_new, sum(`total_turnover`) AS total_turnover_new, sum(`volume`) as volume_new FROM quant.stock_1min WHERE `datetime` BETWEEN '2022-06-14 00:00:00' AND '2022-06-14 23:59:59' GROUP BY `symbol`, resampling_stock(`datetime`, '5min');
SELECT symbol, max(`datetime`) AS datetime_new, argMin(`open`, `datetime`) AS open_new, max(`high`) AS high_new, min(`low`) AS low_new, argMax(`close`, `datetime`) AS close_new, sum(`total_turnover`) AS total_turnover_new, sum(`volume`) as volume_new FROM quant.stock_5min WHERE `datetime` BETWEEN '2022-06-14 00:00:00' AND '2022-06-14 23:59:59' GROUP BY `symbol`, resampling_stock(`datetime`, '15min');
SELECT symbol, max(`datetime`) AS datetime_new, argMin(`open`, `datetime`) AS open_new, max(`high`) AS high_new, min(`low`) AS low_new, argMax(`close`, `datetime`) AS close_new, sum(`total_turnover`) AS total_turnover_new, sum(`volume`) as volume_new FROM quant.stock_15min WHERE `datetime` BETWEEN '2022-06-14 00:00:00' AND '2022-06-14 23:59:59' GROUP BY `symbol`, resampling_stock(`datetime`, '30min');
SELECT symbol, max(`datetime`) AS datetime_new, argMin(`open`, `datetime`) AS open_new, max(`high`) AS high_new, min(`low`) AS low_new, argMax(`close`, `datetime`) AS close_new, sum(`total_turnover`) AS total_turnover_new, sum(`volume`) as volume_new FROM quant.stock_30min WHERE `datetime` BETWEEN '2022-06-14 00:00:00' AND '2022-06-14 23:59:59' GROUP BY `symbol`, resampling_stock(`datetime`, '60min');
```