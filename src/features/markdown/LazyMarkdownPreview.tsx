import { lazy, Suspense } from "react";
import type { MarkdownPreviewProps } from "./MarkdownPreview";

const MarkdownPreviewImpl = lazy(() =>
  import("./MarkdownPreview").then((module) => ({ default: module.MarkdownPreview })),
);

export function LazyMarkdownPreview(props: MarkdownPreviewProps) {
  // contentSnapshot 在 MainEditor 中已经做了 5s 防抖，这里直接传递即可
  return (
    <Suspense fallback={null}>
      <MarkdownPreviewImpl {...props} />
    </Suspense>
  );
}
