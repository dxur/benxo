<script lang="ts">
  import { filePicker } from "../lib/stores/filePicker";
  import Dialog from "./Dialog.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import type { FileEntrySummary } from "@bindings/FileEntrySummary";
  import type { Page } from "@bindings/Page";

  let dialogEl: HTMLDialogElement;
  let files: FileEntrySummary[] = [];
  let selectedFiles: FileEntrySummary[] = [];
  let loading = false;
  let loadingMore = false;
  let error: string | null = null;
  let currentPage: Page<FileEntrySummary> | null = null;
  let hasMore = false;

  $: state = $filePicker;
  $: isOpen = state.isOpen;
  $: allowedMime = state.allowedMime;
  $: multiSelect = state.multiSelect;

  async function fetchFiles(
    mimeFilter?: string[],
    nextToken?: string | null,
    page?: number | null,
    append = false,
  ): Promise<void> {
    try {
      const response: Page<FileEntrySummary> = await ApiRoutes.get_all_files({
        page: page || null,
        per_page: 50, // Set a reasonable page size
        next_token: nextToken || null,
      });

      currentPage = response;
      hasMore = !!response.next_token;

      if (append) {
        files = [...files, ...response.data];
      } else {
        files = response.data;
      }
    } catch (err) {
      console.error("Error fetching files:", err);

      // Handle different types of errors
      if (err instanceof Error) {
        throw new Error(`Failed to fetch files: ${err.message}`);
      } else if (typeof err === "string") {
        throw new Error(`Failed to fetch files: ${err}`);
      } else {
        throw new Error("Failed to fetch files: Unknown error occurred");
      }
    }
  }

  async function loadFiles(): Promise<void> {
    if (!isOpen) return;

    loading = true;
    error = null;
    selectedFiles = [];
    files = [];
    currentPage = null;
    hasMore = false;

    try {
      await fetchFiles(allowedMime);
    } catch (err) {
      if (err instanceof Error) {
        error = err.message;
      } else {
        error = "Failed to load files";
      }
      console.error("Error loading files:", err);
      files = []; // Reset files array on error
    } finally {
      loading = false;
    }
  }

  async function loadMoreFiles(): Promise<void> {
    if (!hasMore || !currentPage || loadingMore) return;

    loadingMore = true;
    error = null;

    try {
      await fetchFiles(
        allowedMime,
        currentPage.next_token,
        currentPage.page ? currentPage.page + 1 : null,
        true,
      );
    } catch (err) {
      if (err instanceof Error) {
        error = err.message;
      } else {
        error = "Failed to load more files";
      }
      console.error("Error loading more files:", err);
    } finally {
      loadingMore = false;
    }
  }

  function handleFileSelect(file: FileEntrySummary): void {
    if (!multiSelect) {
      selectedFiles = [file];
    } else {
      const index = selectedFiles.findIndex((f) => f.url === file.url);
      if (index >= 0) {
        selectedFiles = selectedFiles.filter((f) => f.url !== file.url);
      } else {
        selectedFiles = [...selectedFiles, file];
      }
    }
  }

  function handleConfirm(): void {
    if (multiSelect) {
      filePicker.close(selectedFiles);
    } else {
      filePicker.close(selectedFiles[0]);
    }
  }

  function handleCancel(): void {
    filePicker.cancel();
  }

  function formatFileSize(bytes: number): string {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function getFileIcon(mime: string): string {
    if (mime.startsWith("image/")) return "🖼️";
    if (mime.startsWith("video/")) return "🎥";
    if (mime.startsWith("audio/")) return "🎵";
    if (mime === "application/pdf") return "📄";
    return "📁";
  }

  function getFileName(url: string): string {
    return url.split("/").pop() || "Unknown file";
  }

  // Load files when dialog opens
  $: if (isOpen) {
    loadFiles();
  }

  // Handle dialog element
  $: if (dialogEl && isOpen) {
    dialogEl.showModal();
  } else if (dialogEl && !isOpen) {
    dialogEl.close();
  }
</script>

<div>
  {#if isOpen}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <Dialog
      bind:dialog={dialogEl}
      on:click={(e) => e.target === dialogEl && handleCancel()}
    >
      <div class="dialog-content">
        <header class="dialog-header">
          <h2>Select {multiSelect ? "Files" : "File"}</h2>
          <button class="close-btn" on:click={handleCancel} type="button"
            >×</button
          >
        </header>

        <div class="dialog-body">
          {#if loading}
            <div class="loading">
              <div class="spinner"></div>
              <p>Loading files...</p>
            </div>
          {:else if error}
            <div class="error">
              <div class="error-icon">⚠️</div>
              <h3>Error Loading Files</h3>
              <p>{error}</p>
              <button on:click={loadFiles} type="button" class="retry-btn">
                Try Again
              </button>
            </div>
          {:else if files.length === 0}
            <div class="empty">
              <div class="empty-icon">📁</div>
              <h3>No Files Found</h3>
              <p>
                {#if allowedMime}
                  No files matching "{allowedMime}" were found.
                {:else}
                  No files are available at the moment.
                {/if}
              </p>
              <button on:click={loadFiles} type="button" class="refresh-btn">
                Refresh
              </button>
            </div>
          {:else}
            <div class="file-grid">
              {#each files as file (file.url)}
                <div
                  class="file-item"
                  class:selected={selectedFiles.some((f) => f.url === file.url)}
                  on:click={() => handleFileSelect(file)}
                  role="button"
                  tabindex="0"
                  on:keydown={(e) => {
                    if (e.key === "Enter" || e.key === " ") {
                      e.preventDefault();
                      handleFileSelect(file);
                    }
                  }}
                >
                  <div class="file-icon">
                    <img
                      src={file.url}
                      alt=""
                      style="width: 100%; height: 100%; object-fit: cover; padding: 4px;"
                    />
                  </div>
                  <div class="file-info">
                    <div class="file-name" title={getFileName(file.url)}>
                      {getFileName(file.url)}
                    </div>
                    <div class="file-meta">
                      <span class="file-size"
                        >{formatFileSize(file.size || 0)}</span
                      >
                      {#if file.mime}
                        <span class="file-type">{file.mime}</span>
                      {/if}
                    </div>
                  </div>
                  {#if selectedFiles.some((f) => f.url === file.url)}
                    <div class="selected-indicator">✓</div>
                  {/if}
                </div>
              {/each}
            </div>

            {#if hasMore}
              <div class="load-more-container">
                <button
                  class="btn btn-secondary load-more-btn"
                  on:click={loadMoreFiles}
                  disabled={loadingMore}
                  type="button"
                >
                  {#if loadingMore}
                    <div class="spinner-small"></div>
                    Loading more...
                  {:else}
                    Load More Files
                  {/if}
                </button>
              </div>
            {/if}
          {/if}
        </div>

        <footer class="dialog-footer">
          <div class="footer-info">
            <div class="selected-count">
              {#if multiSelect}
                {selectedFiles.length} file{selectedFiles.length !== 1
                  ? "s"
                  : ""}
                selected
              {:else if selectedFiles.length > 0}
                1 file selected
              {:else}
                No file selected
              {/if}
            </div>
            {#if currentPage && currentPage.total !== null}
              <div class="total-count">
                {files.length} of {currentPage.total} files shown
              </div>
            {/if}
          </div>
          <div class="dialog-actions">
            <button
              class="btn btn-secondary"
              on:click={handleCancel}
              type="button"
            >
              Cancel
            </button>
            <button
              class="btn btn-primary"
              disabled={selectedFiles.length === 0}
              on:click={handleConfirm}
              type="button"
            >
              Select
            </button>
          </div>
        </footer>
      </div>
    </Dialog>
  {/if}
</div>

<style>
  .dialog-content {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #e5e7eb;
  }

  .dialog-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #111827;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 2rem;
    cursor: pointer;
    color: #6b7280;
    padding: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .close-btn:hover {
    background-color: #f3f4f6;
    color: #374151;
  }

  .dialog-body {
    flex: 1;
    padding: 1.5rem;
    overflow-y: auto;
    min-height: 300px;
  }

  .loading,
  .error,
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 300px;
    color: #6b7280;
    text-align: center;
  }

  .error {
    color: #ef4444;
  }

  .error-icon,
  .empty-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .error h3,
  .empty h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .error p,
  .empty p {
    margin: 0 0 1.5rem 0;
    max-width: 400px;
    line-height: 1.5;
  }

  .spinner {
    width: 2rem;
    height: 2rem;
    border: 2px solid #e5e7eb;
    border-top: 2px solid #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .retry-btn,
  .refresh-btn {
    padding: 0.75rem 1.5rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
  }

  .retry-btn:hover,
  .refresh-btn:hover {
    background: #2563eb;
  }

  .file-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }

  .file-item {
    display: flex;
    align-items: center;
    padding: 1rem;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    position: relative;
    background: white;
  }

  .file-item:hover,
  .file-item:focus {
    border-color: #3b82f6;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    outline: none;
  }

  .file-item.selected {
    border-color: #3b82f6;
    background-color: #eff6ff;
  }

  .file-icon {
    width: 72px;
    height: 72px;
    overflow: hidden;
    position: relative;
  }

  .file-info {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-weight: 500;
    color: #111827;
    margin-bottom: 0.25rem;
    word-break: break-all;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-meta {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .file-size,
  .file-type {
    font-size: 0.75rem;
    color: #6b7280;
  }

  .selected-indicator {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    width: 1.5rem;
    height: 1.5rem;
    background: #3b82f6;
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    font-weight: bold;
  }

  .dialog-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-top: 1px solid #e5e7eb;
  }

  .footer-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .selected-count {
    color: #6b7280;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .total-count {
    color: #9ca3af;
    font-size: 0.75rem;
  }

  .load-more-container {
    display: flex;
    justify-content: center;
    margin-top: 2rem;
    padding-top: 1rem;
  }

  .load-more-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .spinner-small {
    width: 1rem;
    height: 1rem;
    border: 2px solid #e5e7eb;
    border-top: 2px solid #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .dialog-actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.2s;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: white;
    color: #374151;
    border-color: #d1d5db;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #f9fafb;
  }

  .btn-primary {
    background: #3b82f6;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2563eb;
  }
</style>
