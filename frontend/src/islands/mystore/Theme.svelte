<script>
    import { setContext } from "svelte";

    let { fonts = true, children } = $props();
    setContext("wx-theme", "modern-dark");
</script>

{#if children}
    <div class="wx-modern-dark-theme" style="height:100%">
        {@render children()}
    </div>
{/if}

<style>
    :global(.wx-modern-dark-theme) {
        /* ========================================
		   GLOBAL DESIGN SYSTEM VARIABLES
		   ======================================== */

        /* Brand Colors - Modern dark theme with teal accents */
        --app-primary-50: oklch(0.35 0.02 185);
        --app-primary-100: oklch(0.4 0.04 185);
        --app-primary-500: oklch(0.65 0.1 185);
        --app-primary-600: oklch(0.6 0.12 185);
        --app-primary-700: oklch(0.55 0.14 185);
        --app-primary-900: oklch(0.3 0.08 185);

        /* Neutral Colors - Adwaita-inspired layered grays */
        --app-gray-50: oklch(0.92 0.005 240);
        --app-gray-100: oklch(0.85 0.005 240);
        --app-gray-200: oklch(0.6 0.01 240);
        --app-gray-300: oklch(0.45 0.015 240);
        --app-gray-400: oklch(0.4 0.015 240);
        --app-gray-500: oklch(0.35 0.02 240);
        --app-gray-600: oklch(0.3 0.015 240);
        --app-gray-700: oklch(0.25 0.015 240);
        --app-gray-800: oklch(0.22 0.01 240);
        --app-gray-900: oklch(0.18 0.01 240);

        /* Semantic Colors - Muted for dark theme comfort */
        --app-success: oklch(0.6 0.1 140);
        --app-warning: oklch(0.65 0.1 90);
        --app-info: oklch(0.6 0.08 220);
        --app-danger: oklch(0.6 0.18 25);

        /* Background Colors - Layered dark grays */
        --app-bg-primary: oklch(0.22 0.01 240);
        --app-bg-secondary: oklch(0.25 0.015 240);
        --app-bg-tertiary: oklch(0.3 0.015 240);
        --app-bg-hover: oklch(0.28 0.015 240);
        --app-bg-disabled: oklch(0.2 0.01 240);
        --app-bg-overlay: oklch(0.1 0.01 240 / 0.7);

        /* Text Colors - Light grays for readability */
        --app-text-primary: oklch(0.92 0.005 240);
        --app-text-secondary: oklch(0.6 0.01 240);
        --app-text-tertiary: oklch(0.45 0.015 240);
        --app-text-disabled: oklch(0.35 0.02 240);
        --app-text-inverse: oklch(0.15 0.01 185);

        /* Border Colors - Subtle but visible */
        --app-border-light: oklch(0.3 0.015 240);
        --app-border-medium: oklch(0.35 0.02 240);
        --app-border-strong: oklch(0.4 0.015 240);

        /* Spacing Scale - Unchanged */
        --app-space-1: 4px;
        --app-space-2: 8px;
        --app-space-3: 12px;
        --app-space-4: 16px;
        --app-space-5: 20px;
        --app-space-6: 24px;
        --app-space-8: 32px;
        --app-space-10: 40px;
        --app-space-12: 48px;

        /* Border Radius - Slightly more rounded for modern feel */
        --app-radius-sm: 6px;
        --app-radius-md: 8px;
        --app-radius-lg: 12px;
        --app-radius-xl: 16px;

        /* Shadows - Darker, more subtle for dark theme */
        --app-shadow-xs: 0 1px 2px 0 oklch(0.1 0.01 240 / 0.3);
        --app-shadow-sm: 0 1px 3px 0 oklch(0.1 0.01 240 / 0.4),
            0 1px 2px -1px oklch(0.1 0.01 240 / 0.4);
        --app-shadow-md: 0 4px 6px -1px oklch(0.1 0.01 240 / 0.3),
            0 2px 4px -2px oklch(0.1 0.01 240 / 0.3);
        --app-shadow-lg: 0 10px 15px -3px oklch(0.1 0.01 240 / 0.3),
            0 4px 6px -4px oklch(0.1 0.01 240 / 0.3);
        --app-shadow-xl: 0 20px 25px -5px oklch(0.1 0.01 240 / 0.3),
            0 8px 10px -6px oklch(0.1 0.01 240 / 0.3);
        --app-shadow-2xl: 0 25px 50px -12px oklch(0.1 0.01 240 / 0.5);

        /* Typography - Modern font stack */
        --app-font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
            "Helvetica Neue", Arial, sans-serif;
        --app-font-size-xs: 12px;
        --app-font-size-sm: 14px;
        --app-font-size-md: 16px;
        --app-font-size-lg: 18px;
        --app-font-size-xl: 20px;
        --app-line-height-xs: 16px;
        --app-line-height-sm: 20px;
        --app-line-height-md: 24px;
        --app-line-height-lg: 28px;
        --app-line-height-xl: 28px;
        --app-font-weight-normal: 400;
        --app-font-weight-medium: 500;
        --app-font-weight-semibold: 600;
        --app-font-weight-bold: 700;

        /* Component Sizes - Unchanged */
        --app-size-sm: 32px;
        --app-size-md: 40px;
        --app-size-lg: 48px;

        /* ========================================
		   THEME VARIABLE MAPPINGS
		   ======================================== */

        /* Base colors - mapped to global system */
        --wx-color-primary: var(--app-primary-500);
        --wx-color-primary-selected: var(--app-primary-50);
        --wx-color-primary-font: var(--app-text-inverse);
        --wx-color-secondary: transparent;
        --wx-color-secondary-hover: oklch(0.65 0.1 185 / 0.15);
        --wx-color-secondary-font: var(--app-primary-500);
        --wx-color-secondary-font-hover: var(--app-primary-600);
        --wx-color-secondary-border: var(--app-primary-500);
        --wx-color-secondary-border-disabled: var(--app-border-medium);

        --wx-color-success: var(--app-success);
        --wx-color-warning: var(--app-warning);
        --wx-color-info: var(--app-info);
        --wx-color-danger: var(--app-danger);
        --wx-color-disabled: var(--app-bg-disabled);
        --wx-color-disabled-alt: var(--app-bg-tertiary);

        --wx-color-font: var(--app-text-primary);
        --wx-color-font-alt: var(--app-text-secondary);
        --wx-color-font-disabled: var(--app-text-disabled);
        --wx-color-link: var(--app-primary-500);

        --wx-background: var(--app-bg-primary);
        --wx-background-alt: var(--app-bg-secondary);
        --wx-background-hover: var(--app-bg-hover);

        /* Font - mapped to global typography */
        --wx-font-family: var(--app-font-family);
        --wx-font-size: var(--app-font-size-sm);
        --wx-line-height: var(--app-line-height-sm);
        --wx-font-size-md: var(--app-font-size-sm);
        --wx-line-height-md: var(--app-line-height-md);
        --wx-font-size-hd: var(--app-font-size-xl);
        --wx-line-height-hd: var(--app-line-height-xl);
        --wx-font-size-sm: var(--app-font-size-xs);
        --wx-line-height-sm: var(--app-line-height-xs);
        --wx-font-weight: var(--app-font-weight-normal);
        --wx-font-weight-md: var(--app-font-weight-semibold);
        --wx-font-weight-b: var(--app-font-weight-bold);

        /* Icons - mapped to global system */
        --wx-icon-color: var(--app-text-secondary);
        --wx-icon-size: var(--app-line-height-sm);
        --wx-icon-border-radius: var(--app-radius-sm);

        /* Borders & Layout - mapped to global system */
        --wx-border: 1px solid var(--app-border-light);
        --wx-border-radius: var(--app-radius-md);
        --wx-radius-major: var(--app-radius-lg);
        --wx-border-light: 1px solid var(--app-border-light);
        --wx-border-medium: 1px solid var(--app-border-medium);

        --wx-shadow-light: var(--app-shadow-md);
        --wx-shadow-medium: var(--app-shadow-lg);
        --wx-box-shadow: var(--app-shadow-sm);
        --wx-box-shadow-strong: var(--app-shadow-lg);

        --wx-padding: var(--app-space-3);

        /* Field - mapped to global spacing */
        --wx-field-gutter: var(--app-space-4);
        --wx-field-width: 400px;

        /* Input - mapped to global system */
        --wx-input-font-family: var(--app-font-family);
        --wx-input-font-size: var(--app-font-size-sm);
        --wx-input-line-height: var(--app-line-height-sm);
        --wx-input-font-weight: var(--app-font-weight-normal);
        --wx-input-text-align: left;
        --wx-input-font-color: var(--app-text-primary);
        --wx-input-background: var(--app-bg-secondary);
        --wx-input-background-disabled: var(--app-bg-disabled);
        --wx-input-placeholder-color: var(--app-text-secondary);
        --wx-input-border: 1px solid var(--app-border-light);
        --wx-input-border-focus: 2px solid var(--app-primary-500);
        --wx-input-border-disabled: 1px solid var(--app-border-light);
        --wx-input-border-radius: var(--app-radius-md);
        --wx-input-height: var(--app-size-md);
        --wx-input-width: 100%;
        --wx-input-padding: var(--app-space-2) var(--app-space-3);
        --wx-input-icon-indent: var(--app-space-2);
        --wx-input-icon-color: var(--app-text-secondary);
        --wx-input-icon-size: var(--app-line-height-sm);

        /* Multi combo - mapped to global system */
        --wx-multicombo-tag-gap: var(--app-space-1);
        --wx-multicombo-tag-border: 1px solid var(--app-border-light);
        --wx-multicombo-tag-border-radius: var(--app-radius-sm);
        --wx-multicombo-tag-pading: var(--app-space-1) var(--app-space-2);
        --wx-multicombo-tag-background: var(--app-bg-tertiary);

        /* Checkbox and radio - mapped to global system */
        --wx-checkbox-height: var(--app-line-height-sm);
        --wx-checkbox-size: var(--wx-checkbox-height);
        --wx-checkbox-border-width: 2px;
        --wx-checkbox-border-color: var(--app-border-medium);
        --wx-checkbox-border-color-disabled: var(--app-border-light);
        --wx-checkbox-border-radius: var(--app-radius-sm);
        --wx-checkbox-font-family: var(--app-font-family);
        --wx-checkbox-font-size: var(--app-font-size-sm);
        --wx-checkbox-line-height: var(--app-line-height-sm);
        --wx-checkbox-font-weight: var(--app-font-weight-normal);
        --wx-checkbox-font-color: var(--app-text-primary);

        /* Label - mapped to global system */
        --wx-label-width: 100px;
        --wx-label-margin: 0 0 var(--app-space-2);
        --wx-label-padding: 0;
        --wx-label-font-family: var(--app-font-family);
        --wx-label-font-size: var(--app-font-size-sm);
        --wx-label-line-height: var(--app-line-height-sm);
        --wx-label-font-weight: var(--app-font-weight-medium);
        --wx-label-font-color: var(--app-text-primary);

        /* Button - mapped to global system */
        --wx-button-font-family: var(--app-font-family);
        --wx-button-font-size: var(--app-font-size-sm);
        --wx-button-line-height: var(--app-line-height-sm);
        --wx-button-font-weight: var(--app-font-weight-medium);
        --wx-button-text-transform: none;
        --wx-button-font-color: var(--app-text-primary);
        --wx-button-danger-font-color: var(--app-text-inverse);
        --wx-button-background: var(--app-bg-secondary);
        --wx-button-border: 1px solid var(--app-border-light);
        --wx-button-border-radius: var(--app-radius-md);
        --wx-button-height: var(--app-size-md);
        --wx-button-padding: var(--app-space-2) var(--app-space-4);
        --wx-button-icon-indent: var(--app-space-2);
        --wx-button-icon-size: var(--app-line-height-sm);

        /* Segmented - mapped to global system */
        --wx-segmented-background: var(--app-bg-tertiary);
        --wx-segmented-background-hover: var(--app-bg-hover);
        --wx-segmented-border: 1px solid var(--app-border-light);
        --wx-segmented-border-radius: var(--app-radius-md);
        --wx-segmented-padding: var(--app-space-1);

        /* Tabs - mapped to global system */
        --wx-tabs-background: var(--app-bg-secondary);
        --wx-tabs-background-hover: transparent;
        --wx-tabs-hover-border: var(--app-border-medium);
        --wx-tabs-border-width: 1px;
        --wx-tabs-border-radius: var(--app-radius-md);
        --wx-tabs-divider-width: 1px;
        --wx-tabs-divider-height: 60%;
        --wx-tabs-divider-color: var(--app-border-light);
        --wx-tabs-cell-min-width: 100px;
        --wx-tabs-active-background: transparent;
        --wx-tabs-active-color: var(--app-primary-500);
        --wx-tabs-active-border: var(--app-primary-500);

        /* Slider - mapped to global system */
        --wx-slider-height: 14px;
        --wx-slider-primary: var(--app-primary-500);
        --wx-slider-background: var(--app-bg-tertiary);
        --wx-slider-track-height: 4px;
        --wx-slider-track-border-radius: 2px;
        --wx-slider-thumb-size: var(--wx-slider-height);
        --wx-slider-thumb-border: none;
        --wx-slider-thumb-border-disabled: none;
        --wx-slider-thumb-shadow: var(--app-shadow-lg);
        --wx-slider-label-margin: 0 0 var(--app-space-2);
        --wx-slider-label-font-family: var(--app-font-family);
        --wx-slider-label-font-size: var(--app-font-size-sm);
        --wx-slider-label-line-height: var(--app-line-height-sm);
        --wx-slider-label-font-weight: var(--app-font-weight-medium);
        --wx-slider-label-font-color: var(--app-text-primary);

        /* Switch - mapped to global system */
        --wx-switch-primary: var(--app-primary-500);
        --wx-switch-background: var(--app-border-medium);
        --wx-switch-width: 48px;
        --wx-switch-height: 24px;
        --wx-switch-border-width: 0px;
        --wx-switch-border-color: transparent;
        --wx-switch-border-color-disabled: transparent;
        --wx-switch-thumb-offset: 2px;
        --wx-switch-thumb-border: none;
        --wx-switch-thumb-border-disabled: 1px solid var(--app-border-medium);
        --wx-switch-thumb-background: var(--app-bg-primary);
        --wx-switch-thumb-background-disabled: var(--app-bg-disabled);
        --wx-switch-thumb-shadow: var(--app-shadow-sm);

        /* Popup and dropdown - mapped to global system */
        --wx-popup-z-index: 100;
        --wx-popup-background: var(--app-bg-secondary);
        --wx-popup-shadow: var(--app-shadow-lg);
        --wx-popup-border: 1px solid var(--app-border-light);
        --wx-popup-border-radius: var(--app-radius-lg);

        /* Modal - mapped to global system */
        --wx-modal-z-index: 1000;
        --wx-modal-background: var(--app-bg-secondary);
        --wx-modal-shadow: var(--app-shadow-2xl);
        --wx-modal-border: 1px solid var(--app-border-light);
        --wx-modal-border-radius: var(--app-radius-lg);
        --wx-modal-width: 280px;
        --wx-modal-padding: var(--app-space-4) var(--app-space-5);
        --wx-modal-gutter: var(--app-space-4);
        --wx-modal-backdrop: var(--app-bg-overlay);
        --wx-modal-header-font-family: var(--app-font-family);
        --wx-modal-header-font-size: var(--app-font-size-lg);
        --wx-modal-header-line-height: var(--app-line-height-lg);
        --wx-modal-header-font-weight: var(--app-font-weight-semibold);
        --wx-modal-header-font-color: var(--app-text-primary);

        /* Notice - mapped to global system */
        --wx-notice-z-index: 1010;
        --wx-notice-background: var(--app-bg-secondary);
        --wx-notice-shadow: var(--app-shadow-lg);
        --wx-notice-border: 1px solid var(--app-border-light);
        --wx-notice-border-radius: var(--app-radius-lg);
        --wx-notice-margin: var(--app-space-2) var(--app-space-3);
        --wx-notice-font-family: var(--app-font-family);
        --wx-notice-font-size: var(--app-font-size-sm);
        --wx-notice-line-height: var(--app-line-height-sm);
        --wx-notice-font-weight: var(--app-font-weight-normal);
        --wx-notice-font-color: var(--app-text-primary);
        --wx-notice-padding: var(--app-space-4);
        --wx-notice-width: 240px;
        --wx-notice-icon-size: var(--app-line-height-sm);
        --wx-notice-icon-color: var(--app-text-secondary);
        --wx-notice-type-border-width: 4px;
        --wx-notice-type-border-color: var(--app-text-secondary);
        --wx-notice-type-font-color: var(--app-text-primary);
        --wx-notice-type-icon-color: var(--app-text-secondary);
        --wx-notice-type-background-opacity: 0;
        --wx-notice-type-close-hover-opacity: 1;

        /* Calendar - mapped to global system */
        --wx-calendar-padding: var(--app-space-4);
        --wx-calendar-cell-size: var(--app-size-sm);
        --wx-calendar-gap: var(--app-space-1);
        --wx-calendar-line-gap: var(--app-space-1);
        --wx-calendar-border-radius: var(--app-radius-md);
        --wx-calendar-font-family: var(--app-font-family);
        --wx-calendar-font-size: var(--app-font-size-sm);
        --wx-calendar-line-height: var(--app-line-height-sm);
        --wx-calendar-font-weight: var(--app-font-weight-normal);
        --wx-calendar-font-color: var(--app-text-primary);
        --wx-calendar-icon-color: var(--app-text-secondary);
        --wx-calendar-icon-size: var(--app-line-height-sm);
        --wx-calendar-header-font-size: var(--app-font-size-sm);
        --wx-calendar-header-line-height: var(--app-line-height-sm);
        --wx-calendar-header-font-weight: var(--app-font-weight-medium);
        --wx-calendar-controls-font-family: var(--app-font-family);
        --wx-calendar-controls-font-size: var(--app-font-size-sm);
        --wx-calendar-controls-line-height: var(--app-line-height-sm);
        --wx-calendar-controls-font-weight: var(--app-font-weight-normal);
        --wx-calendar-controls-font-color: var(--app-text-primary);
    }

    :global(.wx-modern-dark-theme) {
        font-family: var(--app-font-family);
        font-size: var(--app-font-size-sm);
        line-height: var(--app-line-height-sm);
        font-weight: var(--app-font-weight-normal);
        font-style: normal;
        letter-spacing: -0.011em;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: left;
        color: var(--app-text-primary);
        background: var(--app-bg-primary);
    }

    :global(
            .wx-modern-dark-theme *,
            .wx-modern-dark-theme *:before,
            .wx-modern-dark-theme *:after
        ) {
        box-sizing: border-box;
    }

    /* Modern enhancements with improved dark theme transitions */
    :global(
            .wx-modern-dark-theme button,
            .wx-modern-dark-theme input,
            .wx-modern-dark-theme select,
            .wx-modern-dark-theme textarea
        ) {
        transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    }

    :global(.wx-modern-dark-theme *:focus) {
        outline: 2px solid var(--app-primary-500);
        outline-offset: 2px;
    }

    :global(.wx-modern-dark-theme ::selection) {
        background: var(--app-primary-50);
        color: var(--app-primary-500);
    }

    /* Enhanced hover effects for dark theme */
    :global(.wx-modern-dark-theme button:hover) {
        background: var(--app-bg-hover);
        border-color: var(--app-border-medium);
    }

    :global(.wx-modern-dark-theme input:hover) {
        border-color: var(--app-border-medium);
    }
</style>
