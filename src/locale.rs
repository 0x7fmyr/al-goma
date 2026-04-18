use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UiText {
    NewList,
    ViewEditList,
    AddToDishtabase,
    ViewEditDishtabase,

    //NEW LIST
    NoDishesInDishtabase,
    HowManyDishes,
    OnlyEnterNums,
    Menu,
    GeneratedList,

    ShoppingList,

    //VIEW DISHTABASE
    IngredientsHeader,
    DishtabaseHeader,

    //POP
    Yes,
    No,
    WriteTxt,
    CheckIfWant,
    CheckNumNo,
    CheckCategoryNo,
    CheckNumYes,
    CheckCategoryYes,
    CantFindCategory,
    PleaseChooseOne,
    PleaseChooseCategory,
    GeneratingReplaceOld1,
    GeneratingReplaceOld2,
    GeneratingReplaceOld3,
    DeletingAys1,
    DeletingAys2,
    PPEnterDishName,
    PPEnterIngredient,

    //CATEGORIES
    Vegetables,
    Fruit,
    Dairy,
    Protein,
    Pantry,
    Spices,
    Misc,

    //TOOLTIPS
    TTInputBox,
    TTViewingDb,
    TTEditingDish,
    TTEditingIngDName,
    TTPopUp,
    TTShowGenList,
    TTShowShoppingList,
    TTPromtPrint,
}

pub fn swedish() -> HashMap<UiText, &'static str> {
    HashMap::from([
        // LEFT MAIN ACTIONS
        (UiText::NewList, "Ny Lista"),
        (UiText::ViewEditList, "Se/Ändra Lista"),
        (UiText::AddToDishtabase, "Lägg till Rätt i Matabas"),
        (UiText::ViewEditDishtabase, "Se/Ändra Matabas"),
        //NEW LIST TEXT
        (UiText::NoDishesInDishtabase, "Inga Rätter i Matabas!"),
        (UiText::HowManyDishes, "Hur Många Rätter?"),
        (UiText::OnlyEnterNums, "Ange bara Siffror"),
        (UiText::Menu, "Menu"),
        (UiText::GeneratedList, "Genererad Lista"),
        (UiText::ShoppingList, "Shoppinglista:"),
        //VIEW DISHTABASE
        (UiText::IngredientsHeader, "Ingredienser:"),
        (UiText::DishtabaseHeader, "Matabas"),
        //POP
        (UiText::Yes, "Ja"),
        (UiText::No, "Nej"),
        (UiText::WriteTxt, "Exportera txt"),
        (UiText::CheckIfWant, "Markera om du vill ha:"),
        (UiText::CheckNumNo, "[ ] Nummer"),
        (UiText::CheckCategoryNo, "[ ] Kategorier"),
        (UiText::CheckNumYes, "[x] Nummer"),
        (UiText::CheckCategoryYes, "[x] Kategorier"),
        (UiText::CantFindCategory, "Jag kan inte hitta kategori för:"),
        (UiText::PleaseChooseOne, "Välj en:"),
        (UiText::PleaseChooseCategory, "Välj en kategori för:"),
        (
            UiText::GeneratingReplaceOld1,
            "Att generera en ny lista kommer",
        ),
        (UiText::GeneratingReplaceOld2, "ta bort den gamla."),
        (UiText::GeneratingReplaceOld3, "Gör ny lista?"),
        (UiText::DeletingAys1, "Tar Bort:"),
        (UiText::DeletingAys2, "Är du säker?"),
        (UiText::PPEnterDishName, "Skriv Rättens Namn"),
        (UiText::PPEnterIngredient, "Lägg till Ingrediens"),
        //CATEGORIES
        (UiText::Vegetables, "Grönsaker"),
        (UiText::Fruit, "Frukt"),
        (UiText::Dairy, "Mejeri"),
        (UiText::Protein, "Protein"),
        (UiText::Pantry, "Skafferi/Torrvaror"),
        (UiText::Spices, "Kryddor"),
        (UiText::Misc, "Annat"),
        //TOOLTIPS
        (
            UiText::TTInputBox,
            "[enter] bekräfta   [del] ta bort   [ctrl+s] spara   [esc] avbryt",
        ),
        (
            UiText::TTViewingDb,
            "[up/down] välj   [enter] redigera   [esc] avbryt",
        ),
        (
            UiText::TTEditingDish,
            "[up/down] välj [enter] redigera [ctrl+n] namn [ctrl+a] lägg till  [ctrl+k] kategori [del] ta bort [esc] avbryt",
        ),
        (UiText::TTEditingIngDName, "[enter] bekräfta   [esc] avbryt"),
        (
            UiText::TTPopUp,
            "[up/down] välj   [enter] bekräfta   [esc] avbryt",
        ),
        (
            UiText::TTShowGenList,
            "[enter] accept   [del] ny rätt   [esc] avbryt",
        ),
        (
            UiText::TTShowShoppingList,
            "[del] ta bort   [ctrl+a] lägg till   [ctrl+p] exportera txt   [esc] avbryt",
        ),
        (
            UiText::TTPromtPrint,
            "[up/down] välj   [enter] bekräfta   [p] exportera   [esc] avbryt",
        ),
    ])
}

pub fn english() -> HashMap<UiText, &'static str> {
    HashMap::from([
        // LEFT MAIN ACTIONS
        (UiText::NewList, "New List"),
        (UiText::ViewEditList, "View/Edit List"),
        (UiText::AddToDishtabase, "Add Dish to Dishtabase"),
        (UiText::ViewEditDishtabase, "View/Edit Dishtabase"),
        //NEW LIST TEXT
        (UiText::NoDishesInDishtabase, "No Dishes in Dishtabase!"),
        (UiText::HowManyDishes, "How many dishes?"),
        (UiText::OnlyEnterNums, "Please only enter Numbers"),
        (UiText::Menu, "Menu"),
        (UiText::GeneratedList, "Generated List"),
        (UiText::ShoppingList, "Shopping List:"),
        //VIEW DISHTABASE
        (UiText::IngredientsHeader, "Ingredients:"),
        (UiText::DishtabaseHeader, "Dishtabase"),
        //POP
        (UiText::Yes, "Yes"),
        (UiText::No, "No"),
        (UiText::WriteTxt, "Write txt"),
        (UiText::CheckIfWant, "Check if you want:"),
        (UiText::CheckNumNo, "[ ] Numbers"),
        (UiText::CheckCategoryNo, "[ ] Categories"),
        (UiText::CheckNumYes, "[x] Numbers"),
        (UiText::CheckCategoryYes, "[x] Categories"),
        (UiText::CantFindCategory, "I can't find a category for:"),
        (UiText::PleaseChooseOne, "Please choose one:"),
        (UiText::PleaseChooseCategory, "Please choose category for:"),
        (UiText::GeneratingReplaceOld1, "Generating a new list will"),
        (UiText::GeneratingReplaceOld2, "delete the old one."),
        (UiText::GeneratingReplaceOld3, "Make new list?"),
        (UiText::DeletingAys1, "Deleting:"),
        (UiText::DeletingAys2, "Are You Sure?"),
        (UiText::PPEnterDishName, "Enter the Dish Name"),
        (UiText::PPEnterIngredient, "Enter Ingredient"),
        //CATEGORIES
        (UiText::Vegetables, "Vegetables"),
        (UiText::Fruit, "Fruit"),
        (UiText::Dairy, "Dairy"),
        (UiText::Protein, "Protein"),
        (UiText::Pantry, "Pantry"),
        (UiText::Spices, "Spices"),
        (UiText::Misc, "Misc"),
        //TOOLTIPS
        (
            UiText::TTInputBox,
            "[enter] confirm   [del] remove   [ctrl+s] save   [esc] cancel",
        ),
        (
            UiText::TTViewingDb,
            "[up/down] select   [enter] edit   [esc] cancel",
        ),
        (
            UiText::TTEditingDish,
            "[up/down] select [enter] edit [ctrl+n] name [ctrl+a] add  [ctrl+k] category [del] remove [esc] cancel",
        ),
        (UiText::TTEditingIngDName, "[enter] confirm   [esc] cancel"),
        (
            UiText::TTPopUp,
            "[up/down] select   [enter] confirm   [esc] cancel",
        ),
        (
            UiText::TTShowGenList,
            "[enter] accept   [del] new dish   [esc] cancel",
        ),
        (
            UiText::TTShowShoppingList,
            "[del] remove   [ctrl+a] add   [ctrl+p] print txt   [esc] cancel",
        ),
        (
            UiText::TTPromtPrint,
            "[up/down] select   [enter] confirm   [p] print   [esc] cancel",
        ),
    ])
}
