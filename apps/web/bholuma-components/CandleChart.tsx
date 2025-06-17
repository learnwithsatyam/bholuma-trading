'use client';

import {
  CandlestickSeries,
  createChart,
  ColorType,
  type CandlestickData,
  type IChartApi,
  type UTCTimestamp,
} from 'lightweight-charts';
import { useEffect, useRef } from 'react';

export default function CandleChart() {
  const containerRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!containerRef.current) return;
    const chartOptions = { layout: { textColor: 'black', background: { type: ColorType.Solid, color: 'white' } } };
    const chart = createChart(containerRef.current, chartOptions);
    const candlestickSeries = chart.addSeries(CandlestickSeries, { upColor: '#26a69a', downColor: '#ef5350', borderVisible: false, wickUpColor: '#26a69a', wickDownColor: '#ef5350' });

    // Fetch from your Rust backend
    fetch('http://localhost:8080/candle-data') // ← Replace with your actual Rust API URL
      .then((res) => res.json())
      .then((data) => {
        const formatted: CandlestickData[] = data.map((d: any) => ({
          time: d[0] as UTCTimestamp,
          open: d[1],
          high: d[2],
          low: d[3],
          close: d[5],
        }));
        candlestickSeries.setData(formatted);
        chart.timeScale().fitContent();
      });

    return () => chart.remove();
  }, []);

  return <div ref={containerRef} />;
}
