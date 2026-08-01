import { cloneElement } from "react";
import type { ReactElement } from "react";
import { Label, Text, YStack } from "tamagui";

export interface FormFieldProps {
  id: string;
  label: string;
  helper?: string;
  error?: string;
  children: ReactElement;
}

export const FormField = ({
  id,
  label,
  helper,
  error,
  children,
}: FormFieldProps) => {
  const helperId = `${id}-helper`;
  const errorId = `${id}-error`;
  const descriptionId = error ? errorId : helper ? helperId : undefined;
  const control = cloneElement(
    children as ReactElement<Record<string, unknown>>,
    {
      id,
      "aria-invalid": Boolean(error),
      "aria-describedby": descriptionId,
    }
  );

  return (
    <YStack gap="$2">
      <Label color="$color" fontWeight="600" htmlFor={id}>
        {label}
      </Label>
      {control}
      {helper ? (
        <Text color="$muted" fontSize="$3" id={helperId}>
          {helper}
        </Text>
      ) : null}
      {error ? (
        <Text color="$error" fontSize="$3" id={errorId}>
          {error}
        </Text>
      ) : null}
    </YStack>
  );
};
