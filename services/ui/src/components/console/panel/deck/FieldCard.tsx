import { createResource, createSignal, Match, Switch } from "solid-js";
import SiteField from "../../../fields/SiteField";
import { getToken } from "../../../site/util";
import { Display, Field } from "../../config/types";
import validator from "validator";
import axios from "axios";

const FieldCard = (props) => {
  const [update, setUpdate] = createSignal(false);

  const toggleUpdate = () => {
    setUpdate(!update());
  }

  return (
    <Switch
      fallback={
        <ViewCard
          card={props.card}
          value={props.value}
          path_params={props.path_params}
          toggleUpdate={toggleUpdate}
        />
      }
    >
      <Match when={update()}>
        <UpdateCard
          card={props.card}
          value={props.value}
          path_params={props.path_params}
          url={props.url}
          toggleUpdate={toggleUpdate}
          handleRefresh={props.handleRefresh}
        />
      </Match>
    </Switch>
  );
};


const ViewCard = (props) => {
  const [is_allowed] = createResource(props.path_params, (path_params) => props.card?.is_allowed?.(path_params));

  return (
    <div class="card">
      <div class="card-header">
        <div class="card-header-title">{props.card?.label}</div>
      </div>
      <div class="card-content">
        <div class="content"><Switch
          fallback={props.value}
        >
          <Match when={props.card?.display === Display.SELECT}>
            {props.card?.field?.value?.options.reduce((field, option) => {
              if (props.value === option.value) {
                return option.option;
              } else {
                return field;
              }
            }, props.value)}
          </Match>
        </Switch></div>
      </div>
      {is_allowed() &&
        <div class="card-footer">
          <a class="card-footer-item" onClick={(e) => {
            e.preventDefault();
            props.toggleUpdate();
          }}>Update</a>
        </div>
      }
    </div >
  );
};


const initForm = (field, value) => {
  switch (field?.kind) {
    case Field.SELECT:
      field.value.selected = value;
      break;
    default:
      field.value = value;
  }

  return {
    [field?.key]: {
      kind: field?.kind,
      label: field?.label,
      value: field?.value,
      valid: field?.valid,
      validate: field?.validate,
      nullify: field?.nullify,
    },
    submitting: false
  };
}

const options = (url: string, token: string, data: any) => {
  return {
    url: url,
    method: "PATCH",
    data: data,
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
  };
};

const UpdateCard = (props) => {
  const [form, setForm] = createSignal(initForm(props.card?.field, props.value));
  const [valid, setValid] = createSignal(false);

  const postData = async (data) => {
    try {
      const token = getToken();
      if (token && !validator.isJWT(token)) {
        return;
      }

      await axios(options(props.url(), token, data));
      props.handleRefresh();
      props.toggleUpdate();
    } catch (error) {
      console.error(error);
    }
  };

  function sendForm(e) {
    e.preventDefault();
    if (is_value_unchanged()) {
      props.toggleUpdate();
      return;
    } else if (!valid() || form()?.submitting) {
      return;
    }
    handleFormSubmitting(true);
    let data = {};
    for (let key of Object.keys(form())) {
      switch (form()?.[key]?.kind) {
        case Field.SELECT:
          data[key] = form()?.[key]?.value?.selected;
          break;
        default:
          if (!form()?.[key]?.value && form()?.[key]?.nullify) {
            data[key] = null;
          } else {
            data[key] = form()?.[key]?.value;
          }
      }
    }
    postData(data);
    handleFormSubmitting(false);
  }

  const is_value_unchanged = () => {
    switch (props.card?.field?.kind) {
      case Field.SELECT:
        if (props.value === form()?.[props.card?.field?.key]?.value?.selected) {
          return true;
        } else {
          return false;
        }
      default:
        if (props.value === form()?.[props.card?.field?.key]?.value) {
          return true;
        } else {
          return false;
        }
    }
  }

  const handleFormSubmitting = (submitting) => {
    setForm({ ...form(), submitting: submitting });
  };

  const handleField = (key, value, valid) => {
    if (key && form()?.[key]) {
      setForm({
        ...form(),
        [key]: {
          ...form()?.[key],
          value: value,
          valid: valid,
        },
      });
      setValid(getValid());
    }
  };

  function getValid() {
    let allValid = true;
    Object.values(form()).forEach((field) => {
      if (field.validate && !field.valid) {
        allValid = false;
      }
    });
    return allValid;
  }

  return (
    <div class="card">
      <div class="card-header">
        <div class="card-header-title">{props.card?.label}</div>
      </div>
      <div class="card-content">
        <div class="content"><SiteField
          kind={props.card?.field?.kind}
          fieldKey={props.card?.field?.key}
          value={form()?.[props.card?.field?.key]?.value}
          valid={form()?.[props.card?.field?.key]?.valid}
          config={props.card?.field?.config}
          handleField={handleField}
        /></div>
      </div>
      <div class="card-footer">
        <a class="card-footer-item"
          onClick={sendForm}
        >Save</a>
        <a class="card-footer-item" onClick={(e) => {
          e.preventDefault();
          props.toggleUpdate();
        }}>Cancel</a>
      </div>
    </div >
  );
};

export default FieldCard;
