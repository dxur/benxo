<script lang="ts">
  import LoadingShow from "@/dashboard/components/LoadingShow.svelte";
  import Editor from "@/dashboard/components/Editor.svelte";
  import Row from "@/dashboard/components/Row.svelte";
  import Content from "@/dashboard/components/Content.svelte";
  import Panel from "@/dashboard/components/Panel.svelte";
  import Card from "@/dashboard/components/Card.svelte";
  import Badge from "@/dashboard/components/Badge.svelte";
  import Table from "@/dashboard/components/Table.svelte";
  import Timeline from "@/dashboard/components/Timeline.svelte";
  import Dialog from "@/dashboard/components/Dialog.svelte";
  import { notifCenter } from "@/dashboard/stores/notifications";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { AppRoutes } from "@/dashboard/routes";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { useNavigate } from "@dvcol/svelte-simple-router/router";
  import { deepEqual, formatDate, isNone } from "@/dashboard/lib/utils";
  import type { LoadingStatus } from "@/dashboard/components/LoadingShow.svelte";
  import type { OrderPublic } from "@bindings/OrderPublic";
  import type { OrderUpdateBody } from "@bindings/OrderUpdateBody";
  import type { OrderStatus } from "@bindings/OrderStatus";
  import type { DeliveryType } from "@bindings/DeliveryType";
  import type { CartItem } from "@bindings/CartItem";

  interface CartItemEntry {
    sku: string;
    quantity: number;
    price: number;
  }

  const { replace } = useNavigate();
  const { location } = useRoute();

  let dialog: HTMLDialogElement;

  let oid: string;
  let status: LoadingStatus = undefined;
  let order: OrderPublic;

  let fields: {
    edit_basic: boolean;
    edit_shipping: boolean;
    edit_cart: boolean;
    full_name: string;
    phone: string;
    email: string;
    province: string;
    address: string;
    delivery: DeliveryType;
    note: string;
    items: CartItemEntry[];
    progress: OrderStatus | undefined;
    regress: OrderStatus | undefined;
  };

  function pull() {
    ApiRoutes.get_one_order({ id: oid })
      .then((data) => {
        order = data;
        updateFields();
        status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        status = err;
      });
  }

  async function update() {
    try {
      const body = tryUpdate();
      if (isNone(body)) {
        notifCenter.warning("No changes made");
        return;
      }

      const data = await ApiRoutes.update_order({ id: oid, body });
      notifCenter.success("Order updated");
      order = data;
      updateFields();
    } catch (err) {
      notifCenter.error(err);
    }
  }

  function remove() {
    ApiRoutes.delete_order({ id: oid })
      .then((_) => {
        notifCenter.success("Order deleted");
        replace({ path: AppRoutes.PRODUCTS.path });
      })
      .catch((err) => {
        notifCenter.error(err);
      });
  }

  function progress() {
    if (!fields.progress) {
      notifCenter.warning("Catch me if you can!");
      return;
    }
    const body: OrderUpdateBody = {
      status: fields.progress,
      full_name: null,
      phone: null,
      email: null,
      province: null,
      address: null,
      delivery: null,
      note: null,
      items: null,
    };
    ApiRoutes.update_order({ id: oid, body }).then((data) => {
      notifCenter.success("Order updated");
      order = data;
      updateFields();
    });
  }

  function regress() {
    if (!fields.regress) {
      notifCenter.warning("Catch me if you can!");
      return;
    }
    const body: OrderUpdateBody = {
      status: fields.regress,
      full_name: null,
      phone: null,
      email: null,
      province: null,
      address: null,
      delivery: null,
      note: null,
      items: null,
    };
    ApiRoutes.update_order({ id: oid, body }).then((data) => {
      notifCenter.success("Order updated");
      order = data;
      updateFields();
    });
  }

  function updateFields() {
    let items: CartItemEntry[] = Object.keys(order.items).map((sku) => {
      const item = order.items[sku]!;
      return {
        sku,
        price: item.price,
        quantity: item.quantity,
      };
    });

    let progress: OrderStatus | undefined = undefined;
    let regress: OrderStatus | undefined = undefined;

    switch (order.status) {
      case "PENDING":
        progress = "CONFIRMED";
        regress = "CANCELED";
        break;
      case "CONFIRMED":
        progress = "DELIVERED";
        regress = "CANCELED";
        break;
      case "DELIVERED":
        progress = "DONE";
        regress = "RETURNED";
        break;
      case "DONE":
        progress = undefined;
        regress = "CANCELED";
        break;
      case "CANCELED":
        progress = "PENDING";
        regress = undefined;
        break;
      case "RETURNED":
        progress = "DELIVERED";
        regress = "CANCELED";
        break;
    }

    fields = {
      edit_basic: false,
      edit_shipping: false,
      edit_cart: false,
      full_name: order.full_name,
      phone: order.phone,
      email: order.email,
      province: order.province,
      address: order.address,
      delivery: order.delivery,
      note: order.note,
      items,
      progress,
      regress,
    };
  }

  function tryUpdate(): OrderUpdateBody {
    if (fields.edit_basic || fields.edit_shipping || fields.edit_cart) {
      throw "Order is being edited";
    }

    const items = tryGetItems();

    return {
      status: null,
      full_name: fields.full_name === order.full_name ? null : fields.full_name,
      phone: fields.phone === order.phone ? null : fields.phone,
      email: fields.email === order.email ? null : fields.email,
      province: fields.province === order.province ? null : fields.province,
      address: fields.address === order.address ? null : fields.address,
      delivery: fields.delivery === order.delivery ? null : fields.delivery,
      note: fields.note === order.note ? null : fields.note,
      items: deepEqual(items, order.items) ? null : items,
    };
  }

  function tryGetItems(): { [key in string]?: CartItem } {
    let items: { [key in string]?: CartItem } = {};
    fields.items.forEach((item) => {
      items[item.sku] = {
        price: item.price,
        quantity: item.quantity,
      };
    });
    return items;
  }

  $: {
    const urlId = location?.params.oid;
    if (urlId) {
      oid = urlId as string;
      pull();
    } else {
      replace({ path: AppRoutes.PRODUCTS.path });
    }
  }
  $: document.title = `Edit Product ${order?.full_name}`;
</script>

<LoadingShow {status}>
  <form
    on:submit={(ev) => {
      ev.preventDefault();
      update();
    }}
  >
    <Editor>
      <header>
        <div>
          <Row>
            <h2>{order.full_name}</h2>
            <Badge>{order.status}</Badge>
          </Row>
          <h3>{oid}</h3>
        </div>
        <Row>
          <button type="reset" on:click={() => remove()}> Delete </button>
          <button type="submit"> Save </button>
        </Row>
      </header>
      <section>
        <Content>
          <Card>
            <Row>
              <h3>Basic</h3>
              <button
                type="button"
                on:click={() => {
                  fields.edit_basic = !fields.edit_basic;
                }}
              >
                {#if fields.edit_basic}
                  Done{:else}Edit
                {/if}
              </button>
            </Row>
            <fieldset>
              <label>
                Full Name
                <input
                  readonly={!fields.edit_basic}
                  type="text"
                  bind:value={fields.full_name}
                  required
                />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Phone
                <input
                  readonly={!fields.edit_basic}
                  type="tel"
                  bind:value={fields.phone}
                  required
                />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Email
                <input
                  readonly={!fields.edit_basic}
                  type="text"
                  bind:value={fields.email}
                  required
                />
              </label>
            </fieldset>
          </Card>
          <Card>
            <Row>
              <h3>Shipping</h3>
              <button
                type="button"
                on:click={() => {
                  fields.edit_shipping = !fields.edit_shipping;
                }}
              >
                {#if fields.edit_shipping}
                  Done{:else}Edit
                {/if}
              </button>
            </Row>
            <fieldset>
              <label>
                Province
                <input
                  readonly={!fields.edit_shipping}
                  type="text"
                  bind:value={fields.province}
                  required
                />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Address
                <input
                  readonly={!fields.edit_shipping}
                  type="text"
                  bind:value={fields.address}
                  required
                />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Note
                <textarea
                  readonly={!fields.edit_shipping}
                  rows="10"
                  bind:value={fields.note}
                  required
                ></textarea>
              </label>
            </fieldset>
          </Card>
          <Card>
            <Row>
              <h3>Cart</h3>
              <button
                type="button"
                on:click={() => {
                  fields.edit_cart = !fields.edit_cart;
                }}
              >
                {#if fields.edit_cart}
                  Done{:else}Edit
                {/if}
              </button>
            </Row>
            {#if fields.items.length}
              <Table>
                <thead>
                  <tr>
                    <th> SKU </th>
                    <th> Quantity </th>
                    <th> Price </th>
                    <th> Options </th>
                  </tr>
                </thead>
                <tbody>
                  {#each fields.items as item}
                    <tr>
                      <td>{item.sku}</td>
                      <td>
                        <input
                          type="number"
                          readonly={!fields.edit_cart}
                          min="1"
                          step="1"
                          bind:value={item.quantity}
                        />
                      </td>
                      <td>
                        <input
                          type="number"
                          readonly={!fields.edit_cart}
                          min="0"
                          step="0.01"
                          bind:value={item.price}
                        />
                      </td>
                      <td>
                        <button
                          type="reset"
                          disabled={!fields.edit_cart}
                          on:click={() => {}}
                        >
                          Remove
                        </button>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </Table>
            {:else}
              <span>Oops nothing here!</span>
            {/if}
            {#if fields.edit_cart}
              <a
                href={undefined}
                on:click={() => {
                  dialog.showModal();
                }}
              >
                Add another item
              </a>
            {/if}
          </Card>
        </Content>
        <Panel>
          <Card>
            <h3>Progress</h3>
            <Timeline>
              {#each order.history as entry}
                <li>
                  <strong> {entry.status} </strong>
                  <time>{formatDate(entry.time)}</time>
                </li>
              {/each}
            </Timeline>
            {#if fields.progress}
              <button type="button" on:click={() => progress()}>
                Mark as {fields.progress}
              </button>
            {/if}
            {#if fields.regress}
              <button type="reset" on:click={() => regress()}>
                Mark as {fields.regress}
              </button>
            {/if}
          </Card>
        </Panel>
      </section>
    </Editor>
  </form>

  <Dialog bind:dialog>
    <header>
      <h2>Select Product</h2>
    </header>
    <form
      on:submit={(ev) => {
        ev.preventDefault();
      }}
    >
      <fieldset>
        <input type="text" placeholder="Search" />
      </fieldset>
      <fieldset>
        <label>
          Quantity
          <input type="number" required />
        </label>
      </fieldset>
      <button type="button" on:click={() => dialog.close()}>Close</button>
      <button type="submit">Submit</button>
    </form>
  </Dialog>
</LoadingShow>
