# 🧩 Creating a Custom Input Field with TanStack Form + InputLayout

When building forms with [`@tanstack/react-form`](https://tanstack.com/query/latest/docs/forms/overview), it’s often helpful to separate **form logic** from **visual layout**. This allows us to build a consistent UI across the app and reuse styling patterns without duplicating field-specific behavior.
Visit the gist for a complete example of the elements [here](https://gist.github.com/Frans06/62d47bd9494b9cfcd3a9ee72868c0617)

Here’s how I created a custom `Input` component that wraps a field using an `InputLayout`, while keeping things clean, type-safe, and extendable.

---

## ✨ Why Use `InputLayout`?

Instead of embedding all field logic and styles into one component, we:

- **Decouple field logic from UI**
- **Encapsulate shared layout (title, error, icon, etc.)**
- **Retain full power of TanStack’s field API**
- **Avoid generic complexity in components that don’t need it**

---

## 🏗️ InputLayout: The Layout Wrapper

The `InputLayout` is a reusable component that wraps around any input-like UI. It uses TanStack’s `useField` under the hood, and renders children using the field API.

```tsx
// InputLayout.tsx (simplified view)

export const InputLayout = ({ children, ...fieldOptions }) => {
  const fieldApi = useField(fieldOptions);

  return (
    <div className="flex flex-col">
      <label htmlFor={fieldOptions.name}>
        {fieldOptions.title}
        {fieldOptions.required && " *"}
      </label>
      {children(fieldApi)}
      {!fieldApi.state.meta.isValid && (
        <em>{fieldApi.state.meta.errors.join(", ")}</em>
      )}
    </div>
  );
};
```

> ✅ Tip: `InputLayout` receives all the field configuration, and renders layout concerns like `title`, `error`, and `note`, so our actual `Input` component stays clean.

---

## 🧠 Custom Input Component

Now we can define our `Input` field component. It's fully compatible with `@tanstack/react-form` and styled with Tailwind:

```tsx
export const Input = ({
  title,
  name,
  inputRef,
  required = true,
  autoFocus = false,
  type,
  placeholder,
  autoComplete = type,
  icon,
  extraInfo,
  note,
  className,
  step = 1,
  min = 0,
  form,
  onKeyPress,
}: InputProps & InputLayoutProps & { form: any }) => {
  const extraProps =
    type === "number"
      ? {
          onKeyPress: (event) => {
            if (!/^\d*\.?\d*$/.test(event.key)) {
              event.preventDefault();
            }
          },
        }
      : { onKeyPress };

  return (
    <InputLayout
      name={name}
      title={title}
      extraInfo={extraInfo}
      required={required}
      icon={icon}
      note={note}
      form={form}
    >
      {(field) => (
        <input
          {...extraProps}
          id={field.name}
          name={field.name}
          value={field.state.value}
          onBlur={field.handleBlur}
          onChange={(e) => field.handleChange(e.target.value)}
          ref={inputRef}
          required={required}
          autoFocus={autoFocus}
          type={type}
          step={step}
          min={min}
          autoComplete={autoComplete}
          placeholder={placeholder ?? ""}
          className={twMerge(
            "text-inter bg-bokara-gray-light text-nacre placeholder:text-pearl caret-nacre h-10 w-full rounded-full px-5 text-base",
            className,
          )}
        />
      )}
    </InputLayout>
  );
};
```

---

## 🧪 Usage Example

You can use the custom `Input` like this in your form definition:

```tsx
<form.Form>
  <Input
    form={form}
    name="email"
    title="Email Address"
    placeholder="you@example.com"
    type="email"
    required
  />
</form.Form>
```

---

## 🧼 Final Thoughts

This setup offers a clean separation between **form logic** and **presentation**, making the form system:

- Easy to maintain
- Visually consistent
- Extensible for custom layouts and fields

You can apply this same pattern to create `Select`, `Checkbox`, `Textarea`, etc., each using `InputLayout` for shared UX elements.

---

## 📁 Directory Structure (Optional)

```bash
components/
├── form/
│   ├── Input.tsx         # Our custom Input component
│   ├── InputLayout.tsx   # Generic layout wrapper
│   └── ...other fields
```

---

## 📌 Dependencies

- `@tanstack/react-form`
- `tailwindcss`
- `tailwind-merge`
