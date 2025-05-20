<script lang="ts">
  import Table from "@/dashboard/components/Table.svelte";
  import TablePagination from "@/dashboard/components/TablePagination.svelte";
  import LoadingShow from "@/dashboard/components/LoadingShow.svelte";
  import Dialog from "@/dashboard/components/Dialog.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { writable } from "svelte/store";
  import { useNavigate } from "@dvcol/svelte-simple-router";
  import { AppRoutes } from "@/dashboard/routes";
  import { getOid } from "@/dashboard/lib/utils";
  import { notifCenter } from "@/dashboard/stores/notifications";
  import type { Page } from "@bindings/Page";
  import type { ProductPublic } from "@bindings/ProductPublic";
  import type { LoadingStatus } from "@/dashboard/components/LoadingShow.svelte";

  const { push } = useNavigate();

  let page = writable(1);
  let total = writable(1);
  let dialog: HTMLDialogElement;
  let status: LoadingStatus = undefined;
  let products: Page<ProductPublic>;

  let fields = default_fields();

  function default_fields() {
    return {
      slug: "",
      name: "",
      category: "",
    };
  }

  function pull(p: number) {
    ApiRoutes.get_all_products({
      page: p,
      per_page: null,
    })
      .then((data) => {
        products = data;
        total.update((_) => Math.ceil(data.total / data.per_page));
        status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        status = err;
      });
  }

  function edit(id: string) {
    push({
      path: AppRoutes.PRODUCT.path,
      params: { oid: getOid(id) },
    });
  }

  function create() {
    ApiRoutes.create_product(fields)
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
        pull($page);
      })
      .catch((err) => notifCenter.error(err));
  }

  $: pull($page);
  $: document.title = "Products";
</script>

<LoadingShow {status}>
  <header>
    <h1>Products</h1>
    <button
      on:click={() => {
        fields = default_fields();
        dialog.showModal();
      }}
    >
      New
    </button>
  </header>

  {#if products.data.length}
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
  {:else}
    <span>Oops nothing here!</span>
  {/if}

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
            bind:value={fields.slug}
            pattern="^[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$"
            required
          />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Name
          <input type="text" bind:value={fields.name} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Category
          <input type="text" bind:value={fields.category} required />
        </label>
      </fieldset>
      <button type="button" on:click={() => dialog.close()}>Close</button>
      <button type="submit">Submit</button>
    </form>
  </Dialog>
</LoadingShow>
