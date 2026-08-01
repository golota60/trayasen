import { getCurrentWindow } from "@tauri-apps/api/window";
import type { FormEvent } from "react";
import { useEffect, useState } from "react";
import { Input, Label, Text, XStack, YStack } from "tamagui";
import {
  applyShortcutKey,
  beginShortcutCapture,
  clearShortcutCapture,
  validatePosition,
} from "./features/positions/positionForm";
import type {
  PositionFieldErrors,
  ShortcutCaptureState,
} from "./features/positions/positionForm";
import { createNewElem } from "./rustUtils";
import { AppButton } from "./ui/Button";
import { SurfaceCard } from "./ui/Card";
import { Alert } from "./ui/Feedback";
import { FormField } from "./ui/FormField";
import { PageShell } from "./ui/PageShell";
import { MAX_HEIGHT, MIN_HEIGHT } from "./utils";

const appWindow = getCurrentWindow();

const inputFocusStyle = {
  borderColor: "$accent",
  outlineColor: "$accent",
  outlineOffset: 2,
  outlineStyle: "solid",
  outlineWidth: 2,
} as const;

const NewPositionPage = () => {
  const [name, setName] = useState("");
  const [value, setValue] = useState("7200");
  const [fieldErrors, setFieldErrors] = useState<PositionFieldErrors>({});
  const [formError, setFormError] = useState<string>();
  const [isSubmitting, setSubmitting] = useState(false);
  const [shortcut, setShortcut] =
    useState<ShortcutCaptureState>(clearShortcutCapture);

  useEffect(() => {
    if (shortcut.status !== "capturing") {
      return;
    }

    const handleShortcutKey = (event: KeyboardEvent) => {
      if (event.defaultPrevented) {
        return;
      }

      event.preventDefault();
      setShortcut((current) => applyShortcutKey(current, event));
    };

    document.addEventListener("keydown", handleShortcutKey);
    return () => document.removeEventListener("keydown", handleShortcutKey);
  }, [shortcut.status]);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const validationErrors = validatePosition(name, value);
    setFieldErrors(validationErrors);
    setFormError(undefined);

    if (Object.keys(validationErrors).length > 0) {
      return;
    }

    setSubmitting(true);
    try {
      const response = await createNewElem(name, value, shortcut.value);

      if (response === "duplicate") {
        setFieldErrors({
          name: "A position with that name already exists",
        });
      } else if (response === "success") {
        await appWindow.close();
      }
    } catch (createError) {
      setFormError(`Could not save position: ${String(createError)}`);
    } finally {
      setSubmitting(false);
    }
  };

  const shortcutButtonText =
    shortcut.status === "idle"
      ? "Click to record"
      : shortcut.status === "capturing"
      ? "Listening for keys…"
      : shortcut.value;

  return (
    <PageShell
      description="Save a desk height and, optionally, assign a keyboard shortcut."
      title="Add a new position"
    >
      <form onSubmit={handleSubmit}>
        <SurfaceCard>
          <FormField
            error={fieldErrors.name}
            id="position-name"
            label="Position name"
          >
            <Input
              autoFocus
              borderColor="$borderColor"
              focusStyle={inputFocusStyle}
              onChangeText={(nextName) => {
                setName(nextName);
                setFieldErrors((current) => ({
                  ...current,
                  name: undefined,
                }));
              }}
              value={name}
            />
          </FormField>

          <FormField
            error={fieldErrors.value}
            helper={`Allowed range: ${MIN_HEIGHT}–${MAX_HEIGHT}`}
            id="position-height"
            label="Position height"
          >
            <Input
              borderColor="$borderColor"
              focusStyle={inputFocusStyle}
              inputMode="numeric"
              onChangeText={(nextValue) => {
                setValue(nextValue);
                setFieldErrors((current) => ({
                  ...current,
                  value: undefined,
                }));
              }}
              value={value}
            />
          </FormField>

          <YStack gap="$2">
            <Label color="$color" fontWeight="600" htmlFor="position-shortcut">
              Keyboard shortcut (optional)
            </Label>
            <XStack
              alignItems="center"
              flexWrap="wrap"
              gap="$3"
              $sm={{ alignItems: "stretch", flexDirection: "column" }}
            >
              <AppButton
                borderColor={
                  shortcut.status === "capturing" ? "$accent" : undefined
                }
                id="position-shortcut"
                onPress={() => setShortcut(beginShortcutCapture())}
                type="button"
                variant="secondary"
              >
                {shortcutButtonText}
              </AppButton>
              {shortcut.value ? (
                <AppButton
                  onPress={() => setShortcut(clearShortcutCapture())}
                  type="button"
                  variant="ghost"
                >
                  Clear
                </AppButton>
              ) : null}
            </XStack>
            {shortcut.status === "capturing" ? (
              <Text aria-live="polite" color="$accent" role="status">
                Listening for shortcut input. Press up to two modifiers and a
                key.
              </Text>
            ) : null}
          </YStack>

          {formError ? (
            <Alert tone="error" title="Could not add position">
              {formError}
            </Alert>
          ) : null}

          <XStack justifyContent="flex-end">
            <AppButton
              loading={isSubmitting}
              loadingLabel="Adding position"
              type="submit"
            >
              Add position
            </AppButton>
          </XStack>
        </SurfaceCard>
      </form>
    </PageShell>
  );
};

export default NewPositionPage;
