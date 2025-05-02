export const testDiffData = [
  {
    leftData: {
      value: {
        lineNumber: 1,
        content: 'import { Button } from "./components/ui";',
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 1,
        content: 'import { Button } from "./components/ui";',
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
  {
    leftData: undefined,
    rightData: {
      value: {
        lineNumber: 2,
        content:
          'import { HighlighterContext, useHighlighterProvider } from "./contexts/highlighter";',
      },
      lang: "tsx",
      diffType: "added",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 2,
        content: "",
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 3,
        content: "",
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 3,
        content: "function App() {",
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 4,
        content: "function App() {",
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
  {
    leftData: undefined,
    rightData: {
      value: {
        lineNumber: 5,
        content: "  const highlighter = useHighlighterProvider();",
      },
      lang: "tsx",
      diffType: "added",
    },
  },
  {
    leftData: undefined,
    rightData: {
      value: {
        lineNumber: 6,
        content: "",
      },
      lang: "tsx",
      diffType: "added",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 4,
        content: "  return (",
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 7,
        content: "  return (",
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 5,
        content: "    <>",
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 8,
        content: "    <>",
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 6,
        content: '      <Button variant="default" size="md">',
      },
      lang: "tsx",
      diffType: "replaced",
    },
    rightData: {
      value: {
        lineNumber: 9,
        content: "      <HighlighterContext.Provider value={{ highlighter }}>",
      },
      lang: "tsx",
      diffType: "replaced",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 7,
        content: "        Default Button",
      },
      lang: "tsx",
      diffType: "replaced",
    },
    rightData: {
      value: {
        lineNumber: 10,
        content: '        <Button variant="default" size="md">',
      },
      lang: "tsx",
      diffType: "replaced",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 8,
        content: "      </Button>",
      },
      lang: "tsx",
      diffType: "replaced",
    },
    rightData: {
      value: {
        lineNumber: 11,
        content: "          Default Button",
      },
      lang: "tsx",
      diffType: "replaced",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 9,
        content: '      <h1 className=" font-bold underline">Hello world!</h1>',
      },
      lang: "tsx",
      diffType: "replaced",
    },
    rightData: {
      value: {
        lineNumber: 12,
        content: "        </Button>",
      },
      lang: "tsx",
      diffType: "replaced",
    },
  },
  {
    leftData: undefined,
    rightData: {
      value: {
        lineNumber: 13,
        content:
          '        <h1 className=" font-bold underline">Hello world!</h1>',
      },
      lang: "tsx",
      diffType: "added",
    },
  },
  {
    leftData: undefined,
    rightData: {
      value: {
        lineNumber: 14,
        content: "      </HighlighterContext.Provider>",
      },
      lang: "tsx",
      diffType: "added",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 10,
        content: "    </>",
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 15,
        content: "    </>",
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 11,
        content: "  );",
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 16,
        content: "  );",
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
  {
    leftData: {
      value: {
        lineNumber: 12,
        content: "}",
      },
      lang: "tsx",
      diffType: "equal",
    },
    rightData: {
      value: {
        lineNumber: 17,
        content: "}",
      },
      lang: "tsx",
      diffType: "equal",
    },
  },
]