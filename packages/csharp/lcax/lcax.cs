namespace LCAx
{

    public partial class Lcax
    {
        public Uri Schema { get; set; }
        public string Title { get; set; }
        public string Type { get; set; }
        public string[] LcaxRequired { get; set; }
        public LcaxProperties Properties { get; set; }
        public Definitions Definitions { get; set; }
    }

    public partial class Definitions
    {
        public Assembly Assembly { get; set; }
        public ClassificationClass Classification { get; set; }
        public Conversion Conversion { get; set; }
        public DistanceUnit DistanceUnit { get; set; }
        public Epd Epd { get; set; }
        public EpdPart EpdPart { get; set; }
        public EpdSource EpdSource { get; set; }
        public ExternalEpd ExternalEpd { get; set; }
        public ImpactCategory ImpactCategory { get; set; }
        public DistanceUnit ImpactCategoryKey { get; set; }
        public InternalEpd InternalEpd { get; set; }
        public DistanceUnit LifeCycleStage { get; set; }
        public Source Source { get; set; }
        public DistanceUnit Standard { get; set; }
        public DistanceUnit SubType { get; set; }
        public Transport Transport { get; set; }
        public DistanceUnit TransportType { get; set; }
        public DistanceUnit Unit { get; set; }
    }

    public partial class Assembly
    {
        public string Type { get; set; }
        public string[] AssemblyRequired { get; set; }
        public AssemblyProperties Properties { get; set; }
    }

    public partial class AssemblyProperties
    {
        public ClassificationSystem Category { get; set; }
        public Classification Classification { get; set; }
        public ClassificationSystem Comment { get; set; }
        public Description Description { get; set; }
        public Description Id { get; set; }
        public MetaData MetaData { get; set; }
        public Description Name { get; set; }
        public Parts Parts { get; set; }
        public Quantity Quantity { get; set; }
        public Results Results { get; set; }
        public AdditionalProperties Unit { get; set; }
    }

    public partial class ClassificationSystem
    {
        public ClassificationSystemType[] Type { get; set; }
    }

    public partial class Classification
    {
        public string[] Type { get; set; }
        public AdditionalProperties Items { get; set; }
    }

    public partial class AdditionalProperties
    {
        public string Ref { get; set; }
    }

    public partial class Description
    {
        public ClassificationSystemType Type { get; set; }
    }

    public partial class MetaData
    {
        public string[] Type { get; set; }
        public Description AdditionalProperties { get; set; }
    }

    public partial class Parts
    {
        public string Type { get; set; }
        public AdditionalProperties AdditionalProperties { get; set; }
    }

    public partial class Quantity
    {
        public string Type { get; set; }
        public string Format { get; set; }
    }

    public partial class Results
    {
        public string[] Type { get; set; }
        public ResultsAdditionalProperties AdditionalProperties { get; set; }
    }

    public partial class ResultsAdditionalProperties
    {
        public string Type { get; set; }
        public Quantity AdditionalProperties { get; set; }
    }

    public partial class ClassificationClass
    {
        public string Type { get; set; }
        public string[] ClassificationRequired { get; set; }
        public ClassificationProperties Properties { get; set; }
    }

    public partial class ClassificationProperties
    {
        public Description Code { get; set; }
        public Description Name { get; set; }
        public Description System { get; set; }
    }

    public partial class Conversion
    {
        public string Type { get; set; }
        public string[] ConversionRequired { get; set; }
        public ConversionProperties Properties { get; set; }
    }

    public partial class ConversionProperties
    {
        public AdditionalProperties To { get; set; }
        public Quantity Value { get; set; }
    }

    public partial class DistanceUnit
    {
        public ClassificationSystemType Type { get; set; }
        public string[] Enum { get; set; }
    }

    public partial class Epd
    {
        public string Type { get; set; }
        public string[] EpdRequired { get; set; }
        public EpdProperties Properties { get; set; }
    }

    public partial class EpdProperties
    {
        public Adpe Adpe { get; set; }
        public Adpe Adpf { get; set; }
        public Adpe Ap { get; set; }
        public ClassificationSystem Comment { get; set; }
        public Classification Conversions { get; set; }
        public Adpe Cru { get; set; }
        public AdditionalProperties DeclaredUnit { get; set; }
        public Adpe Eee { get; set; }
        public Adpe Eet { get; set; }
        public Adpe Ep { get; set; }
        public Description FormatVersion { get; set; }
        public Adpe Fw { get; set; }
        public Adpe Gwp { get; set; }
        public Adpe Hwd { get; set; }
        public Description Id { get; set; }
        public Description Location { get; set; }
        public Adpe Mer { get; set; }
        public MetaData MetaData { get; set; }
        public Adpe Mrf { get; set; }
        public Description Name { get; set; }
        public Adpe Nhwd { get; set; }
        public Adpe Nrsf { get; set; }
        public Adpe Odp { get; set; }
        public Adpe Penre { get; set; }
        public Adpe Penrm { get; set; }
        public Adpe Penrt { get; set; }
        public Adpe Pere { get; set; }
        public Adpe Perm { get; set; }
        public Adpe Pert { get; set; }
        public Adpe Pocp { get; set; }
        public Quantity PublishedDate { get; set; }
        public LifeSpan ReferenceServiceLife { get; set; }
        public Adpe Rsf { get; set; }
        public Adpe Rwd { get; set; }
        public Adpe Sm { get; set; }
        public Adpe Source { get; set; }
        public AdditionalProperties Standard { get; set; }
        public AdditionalProperties Subtype { get; set; }
        public Quantity ValidUntil { get; set; }
        public Description Version { get; set; }
    }

    public partial class Adpe
    {
        public AnyOf[] AnyOf { get; set; }
    }

    public partial class AnyOf
    {
        public Ref? Ref { get; set; }
        public ClassificationSystemType? Type { get; set; }
    }

    public partial class LifeSpan
    {
        public string[] Type { get; set; }
        public string Format { get; set; }
        public double Minimum { get; set; }
    }

    public partial class EpdPart
    {
        public string Type { get; set; }
        public string[] EpdPartRequired { get; set; }
        public EpdPartProperties Properties { get; set; }
    }

    public partial class EpdPartProperties
    {
        public AdditionalProperties EpdSource { get; set; }
        public Description Id { get; set; }
        public MetaData MetaData { get; set; }
        public Description Name { get; set; }
        public Quantity PartQuantity { get; set; }
        public AdditionalProperties PartUnit { get; set; }
        public Quantity ReferenceServiceLife { get; set; }
        public Adpe Transport { get; set; }
    }

    public partial class EpdSource
    {
        public OneOf[] OneOf { get; set; }
    }

    public partial class OneOf
    {
        public string Type { get; set; }
        public string[] OneOfRequired { get; set; }
        public OneOfProperties Properties { get; set; }
        public bool AdditionalProperties { get; set; }
    }

    public partial class OneOfProperties
    {
        public AdditionalProperties Epd { get; set; }
        public AdditionalProperties Externalepd { get; set; }
        public AdditionalProperties Internalepd { get; set; }
    }

    public partial class ExternalEpd
    {
        public string Type { get; set; }
        public string[] ExternalEpdRequired { get; set; }
        public ExternalEpdProperties Properties { get; set; }
    }

    public partial class ExternalEpdProperties
    {
        public Description Format { get; set; }
        public Description Url { get; set; }
        public ClassificationSystem Version { get; set; }
    }

    public partial class ImpactCategory
    {
        public string Type { get; set; }
        public ImpactCategoryProperties Properties { get; set; }
    }

    public partial class ImpactCategoryProperties
    {
        public A1A3 A1A3 { get; set; }
        public A1A3 A4 { get; set; }
        public A1A3 A5 { get; set; }
        public A1A3 B1 { get; set; }
        public A1A3 B2 { get; set; }
        public A1A3 B3 { get; set; }
        public A1A3 B4 { get; set; }
        public A1A3 B5 { get; set; }
        public A1A3 B6 { get; set; }
        public A1A3 B7 { get; set; }
        public A1A3 C1 { get; set; }
        public A1A3 C2 { get; set; }
        public A1A3 C3 { get; set; }
        public A1A3 C4 { get; set; }
        public A1A3 D { get; set; }
    }

    public partial class A1A3
    {
        public A1A3Type[] Type { get; set; }
        public Format Format { get; set; }
    }

    public partial class InternalEpd
    {
        public string Type { get; set; }
        public string[] InternalEpdRequired { get; set; }
        public InternalEpdProperties Properties { get; set; }
    }

    public partial class InternalEpdProperties
    {
        public Description Path { get; set; }
    }

    public partial class Source
    {
        public string Type { get; set; }
        public string[] SourceRequired { get; set; }
        public SourceProperties Properties { get; set; }
    }

    public partial class SourceProperties
    {
        public Description Name { get; set; }
        public ClassificationSystem Url { get; set; }
    }

    public partial class Transport
    {
        public string Type { get; set; }
        public string[] TransportRequired { get; set; }
        public TransportProperties Properties { get; set; }
    }

    public partial class TransportProperties
    {
        public Quantity Distance { get; set; }
        public AdditionalProperties DistanceUnit { get; set; }
        public Description Id { get; set; }
        public Description Name { get; set; }
        public AdditionalProperties TransportType { get; set; }
    }

    public partial class LcaxProperties
    {
        public ClassificationSystem ClassificationSystem { get; set; }
        public ClassificationSystem Comment { get; set; }
        public Description Description { get; set; }
        public Parts EmissionParts { get; set; }
        public Description FormatVersion { get; set; }
        public Description Id { get; set; }
        public ImpactCategories ImpactCategories { get; set; }
        public ClassificationSystem LciaMethod { get; set; }
        public ImpactCategories LifeCycleStages { get; set; }
        public LifeSpan LifeSpan { get; set; }
        public Description Location { get; set; }
        public MetaData MetaData { get; set; }
        public Description Name { get; set; }
        public Results Results { get; set; }
    }

    public partial class ImpactCategories
    {
        public string Type { get; set; }
        public AdditionalProperties Items { get; set; }
    }

    public enum ClassificationSystemType { Null, String };

    public enum Ref { DefinitionsImpactCategory, DefinitionsSource, DefinitionsTransport };

    public enum Format { Double };

    public enum A1A3Type { Null, Number };
}
