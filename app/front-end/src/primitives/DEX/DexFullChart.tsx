import React, { useEffect, useRef } from 'react';
import { Chart } from 'chart.js';
import { Near } from 'near-api-js';

interface DexFullChartProps {
    accountId: string;
}

const DexFullChart: React.FC<DexFullChartProps> = ({ accountId }) => {
    const chartRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const fetchChartData = async () => {
            // Fetch chart data using NEAR JS SDK
            const near = new Near();
            const chartData = await near.fetchChartData(accountId);

            // Create chart using Chart.js
            const ctx = chartRef.current?.getContext('2d');
            if (ctx) {
                new Chart(ctx, {
                    type: 'line',
                    data: {
                        labels: chartData.labels,
                        datasets: [
                            {
                                label: 'Price',
                                data: chartData.prices,
                                borderColor: 'blue',
                                fill: false,
                            },
                        ],
                    },
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                    },
                });
            }
        };

        fetchChartData();
    }, [accountId]);

    return <canvas ref={chartRef} />;
};

export default DexFullChart;
