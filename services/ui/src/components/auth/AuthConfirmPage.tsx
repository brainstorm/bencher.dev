import axios from "axios";
import { createEffect, createSignal } from "solid-js";
import { Field } from "../console/config/types";
import { BENCHER_API_URL } from "../console/config/util";
import userFieldsConfig from "../fields/config/user/userFieldsConfig";
import SiteField from "../fields/SiteField";

const AuthConfirmPage = (props: {
  config: any;
  handleTitle: Function;
  handleRedirect: Function;
  handleUser: Function;
  handleNotification: Function;
}) => {
  props.handleTitle(props.config?.title);

  const [form, setForm] = createSignal(initForm());

  createEffect(() => {
    var valid = form()?.token?.valid;
    if (valid !== form()?.valid) {
      setForm({ ...form(), valid: valid });
    }
  });

  const handleField = (key, value, valid) => {
    setForm({
      ...form(),
      [key]: {
        value: value,
        valid: valid,
      },
    });
  };

  const handleFormSubmit = (event) => {
    event.preventDefault();
    handleFormSubmitting(true);
    const json_data = {
      token: form()?.token?.value,
    };
    fetchData(json_data)
      .then((resp) => {
        console.log(resp);
        props.handleUser(resp.data);
        props.handleNotification({ status: "ok", text: "Hello" });
        props.handleRedirect(props.config?.form?.redirect);
      })
      .catch((e) => {
        props.handleNotification({
          status: "error",
          text: `Failed to confirm token: ${e}`,
        });
      });
    handleFormSubmitting(false);
  };

  const handleFormSubmitting = (submitting) => {
    setForm({ ...form(), submitting: submitting });
  };

  const request_config = (data) => {
    return {
      url: props.config?.form?.path,
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      data: data,
    };
  };

  const fetchData = async (auth_json) => {
    try {
      const config = request_config(auth_json);
      let resp = await axios(config);
      return resp;
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <section class="section">
      <div class="container">
        <div class="columns is-centered">
          <div class="column is-two-fifths">
            <h2 class="title">{props.config?.title}</h2>
            <h3 class="subtitle">{props.config?.sub}</h3>

            <form class="box">
              <SiteField
                kind={Field.INPUT}
                fieldKey="token"
                label={true}
                value={form()?.token?.value}
                valid={form()?.token?.valid}
                config={userFieldsConfig.token}
                handleField={handleField}
              />

              <button
                class="button is-primary is-fullwidth"
                disabled={!form()?.valid || form()?.submitting}
                onClick={(e) => handleFormSubmit(e)}
              >
                Submit
              </button>
            </form>
          </div>
        </div>
      </div>
    </section>
  );
};

const initForm = () => {
  return {
    token: {
      value: "",
      valid: null,
    },
    valid: false,
    submitting: false,
  };
};

export default AuthConfirmPage;
