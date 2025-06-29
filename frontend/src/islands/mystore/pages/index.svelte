<script lang="ts">
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";
  import { Chart as ChartJS, registerables } from "chart.js";
  import Card from "../components/Card.svelte";
  import { _, format } from "svelte-i18n";

  ChartJS.register(...registerables);

  let salesCanvas: HTMLCanvasElement;
  let usersCanvas: HTMLCanvasElement;
  let ordersCanvas: HTMLCanvasElement;
  let revenueCanvas: HTMLCanvasElement;

  let salesChart: Chart;
  let usersChart: Chart;
  let ordersChart: Chart;
  let revenueChart: Chart;

  // Monthly data
  const months = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
  ];
  const currentMonth = new Date().getMonth();
  const recentMonths = months.slice(currentMonth - 5, currentMonth + 1);

  // Sample data
  const salesData = [12800, 15600, 14300, 18900, 21300, 24500];
  const userData = [850, 1200, 1450, 1800, 2300, 2750];

  // Order status data - will be updated with translations
  let orderStatusData = {
    labels: [] as string[],
    data: [15, 28, 52, 5],
    colors: {
      backgroundColor: [
        "rgba(255, 159, 64, 0.7)",
        "rgba(54, 162, 235, 0.7)",
        "rgba(75, 192, 192, 0.7)",
        "rgba(255, 99, 132, 0.7)",
      ],
      borderColor: [
        "rgb(255, 159, 64)",
        "rgb(54, 162, 235)",
        "rgb(75, 192, 192)",
        "rgb(255, 99, 132)",
      ],
    },
  };

  // Revenue sources data - will be updated with translations
  let revenueSources = {
    labels: [] as string[],
    data: [42, 28, 18, 12],
    colors: {
      backgroundColor: [
        "rgba(75, 192, 192, 0.7)",
        "rgba(153, 102, 255, 0.7)",
        "rgba(255, 159, 64, 0.7)",
        "rgba(54, 162, 235, 0.7)",
      ],
      borderColor: [
        "rgb(75, 192, 192)",
        "rgb(153, 102, 255)",
        "rgb(255, 159, 64)",
        "rgb(54, 162, 235)",
      ],
    },
  };

  // Reactive statements to update chart labels when translations change
  $: {
    orderStatusData.labels = [
      $_("common.status.processing"),
      $_("common.status.shipped"),
      $_("common.status.delivered"),
      $_("common.status.returned"),
    ];
  }

  $: {
    revenueSources.labels = [
      $_("pages.home.charts.revenue.onlineStore"),
      $_("pages.home.charts.revenue.marketplace"),
      $_("pages.home.charts.revenue.socialMedia"),
      $_("pages.home.charts.revenue.retail"),
    ];
  }

  onMount(() => {
    // Sales Chart - Bar Chart with gradient
    const salesCtx = salesCanvas.getContext("2d");
    if (salesCtx) {
      const salesGradient = salesCtx.createLinearGradient(0, 0, 0, 400);
      salesGradient.addColorStop(0, "rgba(55, 125, 255, 0.7)");
      salesGradient.addColorStop(1, "rgba(55, 125, 255, 0.1)");

      salesChart = new Chart(salesCanvas, {
        type: "bar",
        data: {
          labels: recentMonths,
          datasets: [
            {
              label: $_("pages.home.charts.monthlySales"),
              data: salesData,
              backgroundColor: salesGradient,
              borderColor: "rgba(55, 125, 255, 1)",
              borderWidth: 1,
              borderRadius: 8,
              maxBarThickness: 50,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            legend: {
              display: false,
            },
            tooltip: {
              backgroundColor: "rgba(0, 0, 0, 0.7)",
              padding: 10,
              callbacks: {
                label: function (context) {
                  return $_("common.tooltips.currency", {
                    value: context.parsed.y.toLocaleString(),
                  });
                },
              },
            },
          },
          scales: {
            y: {
              beginAtZero: true,
              grid: {
                display: true,
                color: "rgba(0, 0, 0, 0.05)",
              },
              ticks: {
                callback: function (value) {
                  return "$" + value.toLocaleString();
                },
              },
            },
            x: {
              grid: {
                display: false,
              },
            },
          },
        },
      });
    }

    // Users Growth Chart - Line chart with area
    const usersCtx = usersCanvas.getContext("2d");
    if (usersCtx) {
      const usersGradient = usersCtx.createLinearGradient(0, 0, 0, 400);
      usersGradient.addColorStop(0, "rgba(105, 219, 124, 0.5)");
      usersGradient.addColorStop(1, "rgba(105, 219, 124, 0.0)");

      usersChart = new Chart(usersCanvas, {
        type: "line",
        data: {
          labels: recentMonths,
          datasets: [
            {
              label: $_("pages.home.charts.userGrowth"),
              data: userData,
              fill: true,
              backgroundColor: usersGradient,
              borderColor: "rgb(75, 192, 125)",
              borderWidth: 2,
              pointBackgroundColor: "rgb(75, 192, 125)",
              pointBorderColor: "#fff",
              pointRadius: 4,
              pointHoverRadius: 6,
              tension: 0.3,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            legend: {
              display: false,
            },
            tooltip: {
              backgroundColor: "rgba(0, 0, 0, 0.7)",
              padding: 10,
            },
          },
          scales: {
            y: {
              beginAtZero: true,
              grid: {
                color: "rgba(0, 0, 0, 0.05)",
              },
            },
            x: {
              grid: {
                display: false,
              },
            },
          },
        },
      });
    }

    // Orders Status - Doughnut Chart
    ordersChart = new Chart(ordersCanvas, {
      type: "doughnut",
      data: {
        labels: orderStatusData.labels,
        datasets: [
          {
            data: orderStatusData.data,
            backgroundColor: orderStatusData.colors.backgroundColor,
            borderColor: orderStatusData.colors.borderColor,
            borderWidth: 1,
            hoverOffset: 8,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        cutout: "70%",
        plugins: {
          legend: {
            position: "bottom",
            labels: {
              padding: 15,
              usePointStyle: true,
              pointStyle: "circle",
            },
          },
          tooltip: {
            backgroundColor: "rgba(0, 0, 0, 0.7)",
            padding: 10,
            callbacks: {
              label: function (context) {
                const label = context.label || "";
                const value = context.parsed || 0;
                const total = context.dataset.data.reduce(
                  (acc, data) => acc + data,
                  0,
                );
                const percentage = Math.round((value * 100) / total);
                return $_("common.tooltips.percentage", {
                  label,
                  percent: percentage,
                });
              },
            },
          },
        },
      },
    });

    // Revenue Sources - Pie Chart
    revenueChart = new Chart(revenueCanvas, {
      type: "pie",
      data: {
        labels: revenueSources.labels,
        datasets: [
          {
            data: revenueSources.data,
            backgroundColor: revenueSources.colors.backgroundColor,
            borderColor: revenueSources.colors.borderColor,
            borderWidth: 1,
            hoverOffset: 6,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: "bottom",
            labels: {
              padding: 15,
              usePointStyle: true,
              pointStyle: "circle",
            },
          },
          tooltip: {
            backgroundColor: "rgba(0, 0, 0, 0.7)",
            padding: 10,
            callbacks: {
              label: function (context) {
                const label = context.label || "";
                const value = context.parsed || 0;
                const total = context.dataset.data.reduce(
                  (acc, data) => acc + data,
                  0,
                );
                const percentage = Math.round((value * 100) / total);
                return $_("common.tooltips.percentage", {
                  label,
                  percent: percentage,
                });
              },
            },
          },
        },
      },
    });
  });

  // Update document title using translation
  $: document.title = $_("routes.home.title");
</script>

<header>
  <h1>{$_("pages.home.header.title")}</h1>
  <p>{$_("pages.home.header.subtitle")}</p>
</header>

<div class="dashboard-summary">
  <Card>
    <div class="summary-icon">📈</div>
    <div class="summary-data">
      <h3>{$_("pages.home.summary.totalSales.title")}</h3>
      <strong>$107,400</strong>
      <p class="summary-change positive">
        {$_("pages.home.summary.totalSales.change", { value: "15.2" })}
      </p>
    </div>
  </Card>

  <Card>
    <div class="summary-icon">👥</div>
    <div class="summary-data">
      <h3>{$_("pages.home.summary.visitors.title")}</h3>
      <strong>2,750</strong>
      <p class="summary-change positive">
        {$_("pages.home.summary.visitors.change", { value: "19.6" })}
      </p>
    </div>
  </Card>

  <Card>
    <div class="summary-icon">📊</div>
    <div class="summary-data">
      <h3>{$_("pages.home.summary.avgOrderValue.title")}</h3>
      <strong>$87.50</strong>
      <p class="summary-change positive">
        {$_("pages.home.summary.avgOrderValue.change", { value: "5.3" })}
      </p>
    </div>
  </Card>

  <Card>
    <div class="summary-icon">🎯</div>
    <div class="summary-data">
      <h3>{$_("pages.home.summary.conversionRate.title")}</h3>
      <strong>3.8%</strong>
      <p class="summary-change negative">
        {$_("pages.home.summary.conversionRate.changeNegative", {
          value: "0.5",
        })}
      </p>
    </div>
  </Card>
</div>

<div class="charts-container">
  <Card class="sales-chart">
    <header>
      <h3>{$_("pages.home.charts.monthlySales")}</h3>
    </header>
    <div class="chart-canvas-container">
      <canvas bind:this={salesCanvas}></canvas>
    </div>
  </Card>

  <Card class="users-chart">
    <header>
      <h3>{$_("pages.home.charts.userGrowth")}</h3>
    </header>
    <div class="chart-canvas-container">
      <canvas bind:this={usersCanvas}></canvas>
    </div>
  </Card>

  <Card class="orders-chart">
    <header>
      <h3>{$_("pages.home.charts.orderStatus")}</h3>
    </header>
    <div class="chart-canvas-container">
      <canvas bind:this={ordersCanvas}></canvas>
    </div>
  </Card>

  <Card class="revenue-chart">
    <header>
      <h3>{$_("pages.home.charts.revenueSources")}</h3>
    </header>
    <div class="chart-canvas-container">
      <canvas bind:this={revenueCanvas}></canvas>
    </div>
  </Card>
</div>

<style>
  /* Summary Cards */
  .dashboard-summary {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  @media (max-width: 1024px) {
    .dashboard-summary {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 640px) {
    .dashboard-summary {
      grid-template-columns: 1fr;
    }
  }

  .summary-icon {
    font-size: 2rem;
    margin-right: 1rem;
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .summary-data {
    flex: 1;
  }

  .summary-data h3 {
    font-size: 0.875rem;
    font-weight: 500;
    color: #6c757d;
    margin: 0 0 0.5rem 0;
  }

  .summary-change {
    font-size: 0.75rem;
    margin: 0;
  }

  .positive {
    color: #28a745;
  }

  .negative {
    color: #dc3545;
  }

  /* Charts Layout */
  .charts-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(2, 300px);
    gap: 1.5rem;
  }

  @media (max-width: 1024px) {
    .charts-container {
      grid-template-columns: 1fr;
      grid-template-rows: repeat(4, 300px);
    }
  }

  .chart-canvas-container {
    flex: 1;
    position: relative;
    width: 100%;
    height: 100%;
  }
</style>
