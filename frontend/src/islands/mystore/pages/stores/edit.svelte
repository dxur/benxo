<script lang="ts">
  import LoadingShow from "../../components/LoadingShow.svelte";
  import Editor from "../../components/Editor.svelte";
  import Row from "../../components/Row.svelte";
  import Content from "../../components/Content.svelte";
  import Panel from "../../components/Panel.svelte";
  import Card from "../../components/Card.svelte";
  import { notifCenter } from "@/stores/notifications";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { AppRoutes } from "../../routes";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { useNavigate } from "@dvcol/svelte-simple-router/router";
  import type { StorePublic } from "@bindings/StorePublic";
  import type { LoadingStatus } from "../../components/LoadingShow.svelte";
  import type { StoreUpdateBody } from "@bindings/StoreUpdateBody";
  import type { Page } from "@bindings/Page";
  import type { DomainPublic } from "@bindings/DomainPublic";
  import { isNone } from "../../lib/utils";
  import Table from "../../components/Table.svelte";

  const { replace } = useNavigate();
  const { location } = useRoute();

  let oid: string;
  let status: LoadingStatus = undefined;
  let store: StorePublic;
  let domains_status: LoadingStatus = undefined;
  let domains: Page<DomainPublic>;

  let fields: {
    edit_basic: boolean;
    edit_theme: boolean;
    edit_internal: boolean;
    store_id: string;
    name: string;
    description: string;
    primary_color: string;
    secondary_color: string;
    background_color: string;
    head: string;
  };

  function pull() {
    ApiRoutes.get_one_store({ store_id: oid })
      .then((data) => {
        store = data;
        updateFields();
        status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        status = err || "An error occurred";
      });

    ApiRoutes.get_store_domains(
      { store_id: oid },
      {
        page: 1,
        per_page: null,
      },
    )
      .then((data) => {
        domains = data;
        updateFields();
        domains_status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        domains_status = err || "An error occurred";
      });
  }

  async function update() {
    try {
      const body = tryUpdate();
      if (isNone(body)) {
        notifCenter.warning("No changes made");
        return;
      }

      const data = await ApiRoutes.update_store({ store_id: oid, body });
      notifCenter.success("Store updated");
      store = data;
      updateFields();
    } catch (err) {
      notifCenter.error(err);
    }
  }

  function remove() {
    ApiRoutes.delete_store({ store_id: oid })
      .then((_) => {
        notifCenter.success("Store deleted");
        replace({ path: AppRoutes.PRODUCTS.path });
      })
      .catch((err) => {
        notifCenter.error(err);
      });
  }

  function updateFields() {
    fields = {
      edit_basic: false,
      edit_internal: false,
      edit_theme: false,
      store_id: store.store_id,
      name: store.name,
      description: store.description,
      primary_color: store.primary_color,
      secondary_color: store.secondary_color,
      background_color: store.background_color,
      head: store.head,
    };
  }

  function tryUpdate(): StoreUpdateBody {
    if (fields.edit_basic || fields.edit_theme || fields.edit_internal) {
      throw "Store is being edited";
    }

    if (!fields.name || !fields.store_id) {
      throw "Product name can't be empty";
    }

    return {
      store_id: fields.store_id === store.store_id ? null : fields.store_id,
      name: fields.name === store.name ? null : fields.name,
      description:
        fields.description === store.description ? null : fields.description,
      primary_color:
        fields.primary_color === store.primary_color
          ? null
          : fields.primary_color,
      secondary_color:
        fields.secondary_color === store.secondary_color
          ? null
          : fields.secondary_color,
      background_color:
        fields.background_color === store.background_color
          ? null
          : fields.background_color,
      head: fields.head === store.head ? null : fields.head,
    };
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
  $: document.title = `Edit Product ${store?.name}`;
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
          <h2>{store.name}</h2>
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
                Store ID <input
                  type="text"
                  readonly={!fields.edit_basic}
                  bind:value={fields.store_id}
                  required
                /></label
              >
            </fieldset>
            <fieldset>
              <label>
                Name <input
                  type="text"
                  readonly={!fields.edit_basic}
                  bind:value={fields.name}
                  required
                /></label
              >
            </fieldset>
            <fieldset>
              <label>
                Description <textarea
                  readonly={!fields.edit_basic}
                  rows="10"
                  bind:value={fields.description}
                ></textarea></label
              >
            </fieldset>
          </Card>
          <Card>
            <Row>
              <h3>Theme</h3>
              <button
                type="button"
                on:click={() => {
                  fields.edit_theme = !fields.edit_theme;
                }}
              >
                {#if fields.edit_theme}
                  Done{:else}Edit
                {/if}
              </button>
            </Row>
            <fieldset>
              <label>
                Primary Color <input
                  type="color"
                  disabled={!fields.edit_theme}
                  bind:value={fields.primary_color}
                /></label
              >
            </fieldset>
            <fieldset>
              <label>
                Secondary Color <input
                  type="color"
                  disabled={!fields.edit_theme}
                  bind:value={fields.secondary_color}
                /></label
              >
            </fieldset>
            <fieldset>
              <label>
                Background Color <input
                  type="color"
                  disabled={!fields.edit_theme}
                  bind:value={fields.background_color}
                /></label
              >
            </fieldset>
            <fieldset>
              <label> Logo <input disabled={!fields.edit_theme} /></label>
            </fieldset>
          </Card>
          <Card>
            <Row>
              <h3>Internal</h3>
              <button
                type="button"
                on:click={() => {
                  fields.edit_internal = !fields.edit_internal;
                }}
              >
                {#if fields.edit_internal}
                  Done{:else}Edit
                {/if}
              </button>
            </Row>
            <fieldset>
              <label>
                Head Extension<textarea
                  readonly={!fields.edit_internal}
                  rows="10"
                  bind:value={fields.head}
                ></textarea></label
              >
            </fieldset>
          </Card>
        </Content>
        <Panel>
          <Card>
            <Row>
              <h3>Domains</h3>
              <button type="button">Add</button>
            </Row>
            <ul>
              <li>
                <a
                  target="_blank"
                  href={`https://${fields.store_id}.mystore.benxo.test`}
                  >{fields.store_id}.mystore.benxo.test</a
                >
              </li>
              <LoadingShow status={domains_status}>
                {#each domains?.data as domain}
                  <li>
                    <a target="_blank" href={`https://${domain.domain}`}
                      >{domain.domain}</a
                    >
                  </li>
                {/each}
              </LoadingShow>
            </ul>
          </Card>
        </Panel>
      </section>
    </Editor>
  </form>
</LoadingShow>
