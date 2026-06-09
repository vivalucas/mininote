import { lazy, Suspense } from "react";
import type { MarkdownPreviewProps } from "./MarkdownPreview";

const MarkdownPreviewImpl = lazy(() =>
  import("./MarkdownPreview").then((module) => ({ default: module.MarkdownPreview })),
);

export function LazyMarkdownPreview(props: MarkdownPreviewProps) {
  return (
    <Suspense fallback={null}>
      <MarkdownPreviewImpl {...props} />
    </Suspense>
  );
}
