import { lazy, Suspense, useEffect, useState } from "react";
import type { MarkdownPreviewProps } from "./MarkdownPreview";

const MarkdownPreviewImpl = lazy(() =>
  import("./MarkdownPreview").then((module) => ({ default: module.MarkdownPreview })),
);

export function LazyMarkdownPreview(props: MarkdownPreviewProps) {
  const [debouncedContent, setDebouncedContent] = useState(props.content);

  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedContent(props.content);
    }, 150);
    return () => clearTimeout(timer);
  }, [props.content]);

  return (
    <Suspense fallback={null}>
      <MarkdownPreviewImpl {...props} content={debouncedContent} />
    </Suspense>
  );
}
