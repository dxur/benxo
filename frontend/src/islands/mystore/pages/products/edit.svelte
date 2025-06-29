<script lang="ts">
  import LoadingShow from "../../components/LoadingShow.svelte";
  import Editor from "../../components/Editor.svelte";
  import Row from "../../components/Row.svelte";
  import Content from "../../components/Content.svelte";
  import Panel from "../../components/Panel.svelte";
  import Card from "../../components/Card.svelte";
  import Divider from "../../components/Divider.svelte";
  import Badges from "../../components/Badges.svelte";
  import Badge from "../../components/Badge.svelte";
  import Table from "../../components/Table.svelte";
  import { notifCenter } from "@/stores/notifications";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { AppRoutes } from "../../routes";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { useNavigate } from "@dvcol/svelte-simple-router/router";
  import { deepEqual, isNone } from "../../lib/utils";
  import type { ProductPublic } from "@bindings/ProductPublic";
  import type { LoadingStatus } from "../../components/LoadingShow.svelte";
  import type { ProductUpdateBody } from "@bindings/ProductUpdateBody";
  import type { ProductVariant } from "@bindings/ProductVariant";
  import { da } from "@faker-js/faker";

  interface OptionEntry {
    name: string;
    values: Set<string>;
    editing: boolean;
    new_value: string;
  }

  interface VariantEntry {
    sku: string;
    options: Map<string, string>;
    price: number;
    compare_price: number;
    availability: number;
    included: boolean;
  }

  const { replace } = useNavigate();
  const { location } = useRoute();

  let oid: string;
  let status: LoadingStatus = undefined;
  let product: ProductPublic;

  let fields: {
    name: string;
    description: string;
    featured: boolean;
    category: string;
    base_price: number;
    base_compare_price: number;
    base_images: string[];
    options: OptionEntry[];
    variants: VariantEntry[];
    slug: string;
  };

  function pull() {
    ApiRoutes.get_one_product({ id: oid })
      .then((data) => {
        product = data;
        updateFields();
        status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        status = err || "An error occurred";
      });
  }

  async function update() {
    try {
      const body = tryUpdate();
      if (isNone(body)) {
        notifCenter.warning("No changes made");
        return;
      }

      const data = await ApiRoutes.update_product({ id: oid, body });
      notifCenter.success("Product updated");
      product = data;
      updateFields();
    } catch (err) {
      notifCenter.error(err);
    }
  }

  function remove() {
    ApiRoutes.delete_product({ id: oid })
      .then((_) => {
        notifCenter.success("Product deleted");
        replace({ path: AppRoutes.PRODUCTS.path });
      })
      .catch((err) => {
        notifCenter.error(err);
      });
  }

  function addNewOption() {
    fields.options.push({
      name: "",
      values: new Set(),
      editing: true,
      new_value: "",
    });
    fields = fields;
  }

  function removeOption(i: number) {
    fields.options.splice(i, 1);
    updateVariants();
  }

  function doneEditingOption(i: number) {
    const option = fields.options[i];

    if (!option.name || option.values.size === 0) {
      notifCenter.error("Option name and values can't be empty");
      return;
    }
    fields.options[i].editing = false;
    updateVariants();
  }

  function addValue(i: number) {
    let new_option = fields.options[i].new_value.trim();
    if (new_option) {
      fields.options[i].values.add(new_option);
      fields.options[i].new_value = "";
    }
  }

  function removeValue(i: number, value: string) {
    fields.options[i].values.delete(value);
    fields = fields;
  }

  function updateVariants() {
    let valid_variants: VariantEntry[] = fields.variants.filter((variant) => {
      const hasAllOptions = fields.options.every((option) =>
        variant.options.has(option.name),
      );
      const hasValidValues = Array.from(variant.options.entries()).every(
        ([optionName, optionValue]) => {
          const option = fields.options.find((opt) => opt.name === optionName);
          return option && option.values.has(optionValue);
        },
      );
      return hasAllOptions && hasValidValues;
    });

    let variants: VariantEntry[] = [];
    for (const option_set of createVariants(fields.options)) {
      let found = false;
      for (const variant of valid_variants) {
        const matches = option_set.every(
          ([name, value]) =>
            variant.options.has(name) && variant.options.get(name) === value,
        );

        if (matches) {
          found = true;
          variants.push(variant);
          break;
        }
      }

      if (!found) {
        variants.push({
          sku: generateSku(),
          options: new Map(option_set),
          price: product.base_price,
          compare_price: product.base_compare_price,
          availability: Math.floor(Math.random() * 100),
          included: false,
        });
      }
    }
    fields.variants = variants;
  }

  function* createVariants(
    options: OptionEntry[],
  ): Generator<[string, string][], void, unknown> {
    if (options.length === 0) {
      return;
    }
    const [first_option, ...rest_options] = options;
    for (const value of first_option.values) {
      if (rest_options.length === 0) {
        yield [[first_option.name, value]];
      }
      for (const variant of createVariants(rest_options)) {
        yield [[first_option.name, value], ...variant];
      }
    }
  }

  function generateSku() {
    return Array.from(crypto.getRandomValues(new Uint32Array(2)))
      .map((value) => value.toString(36))
      .join("")
      .toUpperCase();
  }

  function updateFields() {
    let options: OptionEntry[] = Object.keys(product.options).map((name) => {
      const values = product.options[name];
      return {
        name: name,
        values: new Set(values),
        editing: false,
        new_value: "",
      };
    });

    let variants: VariantEntry[] = product.variants.map((variant) => ({
      sku: variant.sku,
      options: new Map(
        Object.keys(variant.options).map((name) => [
          name,
          variant.options[name] as string,
        ]),
      ),
      price: variant.price || product.base_price,
      compare_price: variant.compare_price || product.base_compare_price,
      availability: variant.stocks,
      included: true,
    }));

    fields = {
      name: product.name,
      description: product.description,
      featured: product.featured,
      category: product.category,
      base_price: product.base_price,
      base_compare_price: product.base_compare_price,
      base_images: product.base_images,
      slug: product.slug,
      options,
      variants,
    };
    updateVariants();
  }

  function tryUpdate(): ProductUpdateBody {
    if (!fields.name) {
      throw "Product name can't be empty";
    }
    const options = tryGetOptions();
    const variants = tryGetVariants();

    return {
      name: fields.name === product.name ? null : fields.name,
      description:
        fields.description === product.description ? null : fields.description,
      featured: fields.featured === product.featured ? null : fields.featured,
      category: fields.category === product.category ? null : fields.category,
      base_price:
        fields.base_price === product.base_price ? null : fields.base_price,
      base_compare_price:
        fields.base_compare_price === product.base_compare_price
          ? null
          : fields.base_compare_price,
      base_images:
        fields.base_images === product.base_images ? null : fields.base_images,
      slug: fields.slug === product.slug ? null : fields.slug,
      options: deepEqual(options, product.options) ? null : options,
      variants: deepEqual(variants, product.variants) ? null : variants,
    };
  }

  function tryGetOptions(): { [name: string]: string[] } {
    const result: { [name: string]: string[] } = {};
    for (const option of fields.options) {
      if (option.editing) {
        throw "Option is being edited";
      }
      if (!option.name) {
        throw "Option name can't be empty";
      }
      if (option.values.size === 0) {
        throw "Option values can't be empty";
      }
      result[option.name] = Array.from(option.values);
    }
    return result;
  }

  function tryGetVariants(): ProductVariant[] {
    const result = [];

    for (const variant of fields.variants) {
      if (!variant.included) continue;

      if (!variant.sku) {
        throw "Variant SKU can't be empty";
      }

      const price = variant.price === product.base_price ? null : variant.price;
      const comparePrice =
        variant.compare_price === product.base_compare_price
          ? null
          : variant.compare_price;
      const stocks = Math.floor(Math.max(0, variant.availability));

      result.push({
        sku: variant.sku,
        price: price ?? fields.base_price,
        compare_price: comparePrice ?? fields.base_compare_price,
        stocks,
        options: Object.fromEntries(
          Array.from(variant.options).map(([name, value]) => [name, value]),
        ),
        images: [],
      });
    }

    return result;
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
  $: document.title = `Edit Product ${product?.name}`;
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
          <h2>{product.name}</h2>
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
            <h3>Basic</h3>
            <fieldset>
              <label>
                Name <input
                  type="text"
                  bind:value={fields.name}
                  required
                /></label
              >
            </fieldset>
            <fieldset>
              <label>
                Description <textarea rows="10" bind:value={fields.description}
                ></textarea></label
              >
            </fieldset>
          </Card>
          <Card>
            <h3>Media</h3>
            <strong> It's cooking </strong>
          </Card>
          <Card>
            <h3>Pricing</h3>
            <fieldset>
              <Row>
                <label>
                  price <input
                    type="number"
                    bind:value={fields.base_price}
                    step=".01"
                  /></label
                >
                <label>
                  Compare-at price <input
                    type="number"
                    bind:value={fields.base_compare_price}
                    step=".01"
                  /></label
                >
              </Row>
            </fieldset>
          </Card>
          <Card>
            <h3>Options</h3>
            {#if fields.options.length}
              <Divider>
                {#each fields.options as { name, values, new_value, editing }, i}
                  <div>
                    {#if editing}
                      <label>
                        Option name
                        <input
                          type="text"
                          bind:value={name}
                          placeholder="Option name (e.g. Color)"
                        />
                      </label>
                      <label>
                        Option values
                        <fieldset>
                          <div>
                            <input
                              type="text"
                              bind:value={new_value}
                              placeholder="Add value"
                              on:keydown={(ev) => {
                                if (ev.key === "Enter") {
                                  ev.preventDefault();
                                  addValue(i);
                                }
                              }}
                            />
                            <button type="button" on:click={() => addValue(i)}>
                              Add
                            </button>
                          </div>
                          <Badges>
                            {#each values as value}
                              <Badge>
                                <span> {value} </span>
                                <button
                                  type="button"
                                  on:click={() => removeValue(i, value)}
                                  >×</button
                                >
                              </Badge>
                            {/each}
                          </Badges>
                        </fieldset>
                      </label>
                      <Row>
                        <button type="reset" on:click={() => removeOption(i)}>
                          Delete
                        </button>
                        <button
                          type="button"
                          on:click={() => doneEditingOption(i)}
                        >
                          Done
                        </button>
                      </Row>
                    {:else}
                      <Row>
                        <strong> {name} </strong>
                        <button
                          type="button"
                          on:click={() => {
                            editing = true;
                          }}
                        >
                          Edit
                        </button>
                      </Row>
                      <Badges>
                        {#each values as value}
                          <Badge>{value}</Badge>
                        {/each}
                      </Badges>
                    {/if}
                  </div>
                {/each}
              </Divider>
            {/if}
            <a href={undefined} on:click={() => addNewOption()}>
              Add another option
            </a>
          </Card>

          <Card>
            <h3>Variants</h3>
            {#if fields.variants.length}
              <Table>
                <thead>
                  <tr>
                    <th>Combination</th>
                    <th>Included</th>
                    <th>SKU</th>
                    <th>Price</th>
                    <th>Compare-at Price</th>
                    <th>Availability</th>
                  </tr>
                </thead>
                <tbody>
                  {#each fields.variants as variant}
                    <tr>
                      <td>
                        {#each variant.options as [name, value], index}
                          <strong>
                            {value}{#if index !== variant.options.size - 1}/{/if}
                          </strong>
                        {/each}
                      </td>

                      <td>
                        <input
                          type="checkbox"
                          bind:checked={variant.included}
                        />
                      </td>

                      <td>
                        <input
                          type="text"
                          pattern="[A-Z0-9-]*"
                          maxlength="64"
                          bind:value={variant.sku}
                          placeholder="SKU"
                        />
                      </td>

                      <td>
                        <input
                          type="number"
                          placeholder="Price"
                          bind:value={variant.price}
                          step="0.01"
                        />
                      </td>

                      <td>
                        <input
                          type="number"
                          placeholder="Compare-at price"
                          bind:value={variant.compare_price}
                          step="0.01"
                        />
                      </td>

                      <td>
                        <input
                          type="number"
                          placeholder="Availability"
                          bind:value={variant.availability}
                          step="1"
                        />
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </Table>
            {:else}
              <span>Add options to see variants</span>
            {/if}
          </Card>
        </Content>
        <Panel>
          <Card>
            <fieldset>
              <label>
                Slug
                <input
                  type="text"
                  pattern="^[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$"
                  bind:value={fields.slug}
                  required
                />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Featured
                <input type="checkbox" bind:checked={fields.featured} />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Category
                <input type="text" bind:value={fields.category} />
              </label>
            </fieldset>
          </Card>
        </Panel>
      </section>
    </Editor>
  </form>
</LoadingShow>
