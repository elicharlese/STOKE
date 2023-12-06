import React, { useEffect, useRef } from 'react';
import { Chart } from 'chart.js';
import { Near } from 'near-api-js';

interface DexMiniChartProps {
    accountId: string;
}

const DexMiniChart: React.FC<DexMiniChartProps> = ({ accountId }) => {
    const chartRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const fetchData = async () => {
            // Initialize NEAR JS SDK
            const near = new Near();

            // Fetch chart data from DEX API
            const chartData = await near.dex.getChartData(accountId);

            // Create chart
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

        fetchData();
    }, [accountId]);

    return <canvas ref={chartRef} />;
};

export default DexMiniChart;
