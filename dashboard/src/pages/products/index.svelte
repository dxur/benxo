<script lang="ts">
  import Table from "@components/Table.svelte";
  import TablePagination from "@components/TablePagination.svelte";
  import ApiRoutes from "@bindings/ApiRoutes";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { useNavigate } from "@dvcol/svelte-simple-router";
  import { AppRoutes } from "@/routes";
  import { get_oid } from "@lib/utils";
  import { notifCenter } from "@stores/notifications";
  import type { Page } from "@bindings/Page";
  import type { ProductPublic } from "@bindings/ProductPublic";
  import Dialog from "@/components/Dialog.svelte";

  const { push } = useNavigate();

  let products: Page<ProductPublic> | null = null;
  let page = writable(1);
  let total = writable(1);
  let dialog: HTMLDialogElement;

  let slug: string;
  let name: string;
  let category: string;

  function edit(id: string) {
    push({
      path: AppRoutes.PRODUCT.path,
      params: { oid: get_oid(id) },
    });
  }

  function create() {
    ApiRoutes.create_product({ slug, name, category })
      .then((p) => {
        notifCenter.success("Product created successfully");
        edit(p.id);
      })
      .catch((err) => notifCenter.error(err));
  }

  function remove(id: string) {
    ApiRoutes.delete_product({ id })
      .then(() => {
        notifCenter.success("Product deleted");
        ApiRoutes.get_all_products({
          page: $page,
          per_page: 10,
        })
          .then((data) => (products = data))
          .catch((err) => notifCenter.error(err));
      })
      .catch((err) => notifCenter.error(err));
  }

  function pull() {
    ApiRoutes.get_all_products({
      page: 1,
      per_page: 10,
    })
      .then((data) => (products = data))
      .catch((err) => {
        notifCenter.error(err);
      });
  }

  onMount(async () => {
    pull();
  });
</script>

{#if products}
  <header>
    <title> Products </title>
    <h1>Products</h1>
    <button on:click={() => dialog.showModal()}> New </button>
  </header>

  <Table>
    <thead>
      <tr>
        <th>Slug</th>
        <th>Name</th>
        <th>Category</th>
        <th>Featured</th>
        <th>Base Price</th>
        <th>Base Discount</th>
        <th>Options</th>
      </tr>
    </thead>
    <tbody>
      {#each products.data as product}
        <tr>
          <td>{product.slug}</td>
          <td>{product.name}</td>
          <td>{product.category}</td>
          <td>{product.featured ? "Yes" : "No"}</td>
          <td>{product.base_price}</td>
          <td>{product.base_compare_price}</td>
          <td>
            <button on:click={() => edit(product.id)}>Edit</button>
            <button type="reset" on:click={() => remove(product.id)}
              >Delete</button
            >
          </td>
        </tr>
      {/each}
    </tbody>
  </Table>

  <TablePagination {page} {total} />

  <Dialog bind:dialog>
    <header>
      <h2>Create Product</h2>
    </header>
    <form
      on:submit={(ev) => {
        ev.preventDefault();
        create();
      }}
    >
      <fieldset>
        <label>
          Slug
          <input
            type="text"
            bind:value={slug}
            pattern="^[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$"
            required
          />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Name
          <input type="text" bind:value={name} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Category
          <input type="text" bind:value={category} required />
        </label>
      </fieldset>
      <button type="button" on:click={() => dialog.close()}>Close</button>
      <button type="submit">Submit</button>
    </form>
  </Dialog>
{/if}
